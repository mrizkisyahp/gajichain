#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, contracterror, symbol_short,
    Address, BytesN, Env, Symbol, token,
};

// ─────────────────────────────────────────────
//  Storage Keys
// ─────────────────────────────────────────────

const PAYROLL_COUNT: Symbol = symbol_short!("PAY_CNT");
const DISPUTE_COUNT: Symbol = symbol_short!("DIS_CNT");

fn payroll_key(id: u64) -> (Symbol, u64) {
    (symbol_short!("PAYROLL"), id)
}

fn dispute_key(id: u64) -> (Symbol, u64) {
    (symbol_short!("DISPUTE"), id)
}

fn advance_key(payroll_id: u64) -> (Symbol, u64) {
    (symbol_short!("ADVANCE"), payroll_id)
}

// ─────────────────────────────────────────────
//  Types
// ─────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum PaySchedule {
    Daily,
    Weekly,
    Biweekly,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum PayrollStatus {
    Active,
    Released,
    Disputed,
    Cancelled,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum DisputeStatus {
    Open,
    Resolved,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Payroll {
    pub payroll_id: u64,
    pub employer: Address,
    pub worker: Address,
    pub amount: i128,
    pub token: Address,
    pub schedule: PaySchedule,
    pub period_start: u64,
    pub advance_paid: i128,
    pub work_log_hash: BytesN<32>,
    pub work_log_submitted: bool,
    pub status: PayrollStatus,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Dispute {
    pub dispute_id: u64,
    pub payroll_id: u64,
    pub employer: Address,
    pub dispute_reason_hash: BytesN<32>,
    pub stake: i128,
    pub status: DisputeStatus,
}

// ─────────────────────────────────────────────
//  Error Codes
// ─────────────────────────────────────────────

#[contracterror]
#[derive(Clone, Debug, PartialEq)]
pub enum ContractError {
    PayrollNotFound        = 1,
    PayrollAlreadyReleased = 2,
    PayrollAlreadyDisputed = 3,
    PayrollNotActive       = 4,
    WorkLogAlreadySubmitted = 5,
    WorkLogNotSubmitted    = 6,
    UnauthorizedCaller     = 7,
    AdvanceExceedsLimit    = 8,
    InvalidAmount          = 9,
    DisputeNotFound        = 10,
}

// ─────────────────────────────────────────────
//  Contract
// ─────────────────────────────────────────────

#[contract]
pub struct GajiEscrow;

#[contractimpl]
impl GajiEscrow {

    /// Employer deposits salary for a pay period into escrow.
    /// Funds are transferred from the employer to this contract.
    /// Returns the unique payroll_id for this payroll entry.
    pub fn create_payroll(
        env: Env,
        employer: Address,
        worker: Address,
        amount: i128,
        token: Address,
        schedule: PaySchedule,
        period_start: u64,
    ) -> Result<u64, ContractError> {
        // Validate amount
        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        // Employer must authorize this call
        employer.require_auth();

        // Transfer funds from employer into this contract (escrow)
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&employer, &env.current_contract_address(), &amount);

        // Generate next payroll_id
        let payroll_id: u64 = env
            .storage()
            .instance()
            .get(&PAYROLL_COUNT)
            .unwrap_or(0u64)
            + 1;

        // Build payroll record
        let payroll = Payroll {
            payroll_id,
            employer,
            worker,
            amount,
            token,
            schedule,
            period_start,
            advance_paid: 0,
            work_log_hash: BytesN::from_array(&env, &[0u8; 32]),
            work_log_submitted: false,
            status: PayrollStatus::Active,
        };

        // Persist
        env.storage()
            .instance()
            .set(&payroll_key(payroll_id), &payroll);
        env.storage()
            .instance()
            .set(&PAYROLL_COUNT, &payroll_id);

        Ok(payroll_id)
    }

    /// Worker or trusted relayer submits a signed work log for a pay period.
    /// log_hash: SHA-256 of (GPS data || timestamp || QR scan data).
    /// relayer_sig: Ed25519 signature from the trusted relayer over log_hash.
    pub fn submit_work_log(
        env: Env,
        payroll_id: u64,
        worker: Address,
        log_hash: BytesN<32>,
        _relayer_sig: BytesN<64>,
    ) -> Result<(), ContractError> {
        // Worker must authorize this call
        worker.require_auth();

        // Load payroll
        let mut payroll: Payroll = env
            .storage()
            .instance()
            .get(&payroll_key(payroll_id))
            .ok_or(ContractError::PayrollNotFound)?;

        // Guard: must match the registered worker
        if payroll.worker != worker {
            return Err(ContractError::UnauthorizedCaller);
        }

        // Guard: payroll must be active
        if payroll.status != PayrollStatus::Active {
            return Err(ContractError::PayrollNotActive);
        }

        // Guard: only one work log submission allowed per period
        if payroll.work_log_submitted {
            return Err(ContractError::WorkLogAlreadySubmitted);
        }

        // Record log hash and mark as submitted
        payroll.work_log_hash = log_hash;
        payroll.work_log_submitted = true;

        env.storage()
            .instance()
            .set(&payroll_key(payroll_id), &payroll);

        Ok(())
    }

    /// Releases the escrowed payment to the worker.
    /// Can be called by anyone once work log is submitted and payroll is active.
    /// The net payout = amount - advance_paid (recovers any prior salary advance).
    pub fn release_payment(
        env: Env,
        payroll_id: u64,
    ) -> Result<(), ContractError> {
        // Load payroll
        let mut payroll: Payroll = env
            .storage()
            .instance()
            .get(&payroll_key(payroll_id))
            .ok_or(ContractError::PayrollNotFound)?;

        // Guard: must be active
        if payroll.status != PayrollStatus::Active {
            return Err(ContractError::PayrollNotActive);
        }

        // Guard: must not already be released
        if payroll.status == PayrollStatus::Released {
            return Err(ContractError::PayrollAlreadyReleased);
        }

        // Guard: work log must be submitted before release
        if !payroll.work_log_submitted {
            return Err(ContractError::WorkLogNotSubmitted);
        }

        // Calculate net payout (deduct any advance already paid out)
        let net_payout = payroll.amount - payroll.advance_paid;

        // Transfer net payout from contract to worker
        if net_payout > 0 {
            let token_client = token::Client::new(&env, &payroll.token);
            token_client.transfer(
                &env.current_contract_address(),
                &payroll.worker,
                &net_payout,
            );
        }

        // Update status
        payroll.status = PayrollStatus::Released;
        env.storage()
            .instance()
            .set(&payroll_key(payroll_id), &payroll);

        Ok(())
    }

    /// Employer opens a dispute before the auto-release window.
    /// Employer must stake tokens as a dispute bond — returned if they win.
    /// Returns a unique dispute_id for this case.
    pub fn open_dispute(
        env: Env,
        payroll_id: u64,
        employer: Address,
        dispute_reason_hash: BytesN<32>,
        stake: i128,
    ) -> Result<u64, ContractError> {
        // Employer must authorize
        employer.require_auth();

        // Validate stake
        if stake <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        // Load payroll
        let mut payroll: Payroll = env
            .storage()
            .instance()
            .get(&payroll_key(payroll_id))
            .ok_or(ContractError::PayrollNotFound)?;

        // Guard: only the registered employer can open a dispute
        if payroll.employer != employer {
            return Err(ContractError::UnauthorizedCaller);
        }

        // Guard: payroll must be active
        if payroll.status != PayrollStatus::Active {
            return Err(ContractError::PayrollNotActive);
        }

        // Guard: cannot open a second dispute on the same payroll
        if payroll.status == PayrollStatus::Disputed {
            return Err(ContractError::PayrollAlreadyDisputed);
        }

        // Transfer the employer's dispute stake into the contract
        let token_client = token::Client::new(&env, &payroll.token);
        token_client.transfer(&employer, &env.current_contract_address(), &stake);

        // Generate next dispute_id
        let dispute_id: u64 = env
            .storage()
            .instance()
            .get(&DISPUTE_COUNT)
            .unwrap_or(0u64)
            + 1;

        // Build dispute record
        let dispute = Dispute {
            dispute_id,
            payroll_id,
            employer,
            dispute_reason_hash,
            stake,
            status: DisputeStatus::Open,
        };

        // Freeze the payroll into Disputed state
        payroll.status = PayrollStatus::Disputed;

        // Persist both records
        env.storage()
            .instance()
            .set(&dispute_key(dispute_id), &dispute);
        env.storage()
            .instance()
            .set(&payroll_key(payroll_id), &payroll);
        env.storage()
            .instance()
            .set(&DISPUTE_COUNT, &dispute_id);

        Ok(dispute_id)
    }

    /// Worker requests an early wage advance of up to 80% of the total payroll amount.
    /// The advance is deducted from the final payout during release_payment.
    pub fn request_advance(
        env: Env,
        payroll_id: u64,
        worker: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        // Worker must authorize
        worker.require_auth();

        // Validate amount
        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        // Load payroll
        let mut payroll: Payroll = env
            .storage()
            .instance()
            .get(&payroll_key(payroll_id))
            .ok_or(ContractError::PayrollNotFound)?;

        // Guard: must match the registered worker
        if payroll.worker != worker {
            return Err(ContractError::UnauthorizedCaller);
        }

        // Guard: payroll must be active
        if payroll.status != PayrollStatus::Active {
            return Err(ContractError::PayrollNotActive);
        }

        // Guard: total advances must not exceed 80% of the payroll amount
        let max_advance = (payroll.amount * 80) / 100;
        if payroll.advance_paid + amount > max_advance {
            return Err(ContractError::AdvanceExceedsLimit);
        }

        // Existing advance record (cumulative tracking)
        let current_advance: i128 = env
            .storage()
            .instance()
            .get(&advance_key(payroll_id))
            .unwrap_or(0i128);

        // Transfer advance from contract to worker
        let token_client = token::Client::new(&env, &payroll.token);
        token_client.transfer(&env.current_contract_address(), &worker, &amount);

        // Record cumulative advance on payroll and in advance ledger
        payroll.advance_paid += amount;
        env.storage()
            .instance()
            .set(&advance_key(payroll_id), &(current_advance + amount));
        env.storage()
            .instance()
            .set(&payroll_key(payroll_id), &payroll);

        Ok(())
    }
}

mod test;