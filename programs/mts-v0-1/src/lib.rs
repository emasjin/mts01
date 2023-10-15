use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed; 
use anchor_spl::token; 
use anchor_spl::{     
    associated_token::AssociatedToken,
    token::{self,Approve, Mint, MintTo, Revoke, Transfer, Token, TokenAccount},
};

use mpl_token_metadata::{     
    instruction::{freeze_delegated_account, thaw_delegated_account,
                    create_metadata_accounts_v3},
    ID as MetadataTokenId, 
};

declare_id!("1111111111111111111");

#[program]
mod mts_token_pool {
    use super::*;
    // set the mts tokens that mts pool can use
    pub fn set_mts_tokens(ctx: Context<setMTSTokens>, token: Pubkey) -> ProgramResult {
        let mts_tokens = &mut ctx.accounts.mts_tokens;
        mts_tokens.mtokens.push(token);
        Ok(())
    }

    //create MTS pool (MTS means The Martinger Strategy)
    pub fn create_mts_pool(ctx: Context<initMTSPool>, inint_data: MTSPoolPara) -> ProgramResult {
        let mts_pool = &mut ctx.accounts.mts_pool;
        let mtstoken_ = ctx.accounts.minttoken_in_pool;
        // check the pool mint token are in the mtstokens
        let mut check_fial = true;
        for token in mtstoken.mtokens {
            if(token == ctx.accounts.mts_pool.mint_token){
                check_fial = false;
                break;
            }
        }
        require_eq!(check_fial, false, MTSError::MinTokenNotPermitedForMTSPool);
        //
        mts_pool = MTSPoolPara;
        Ok(())
    }


    //用户加入mtspool并向mtspool中转入token
    pub fn join_mtspool(ctx: Context<DepositTokens>) -> ProgramResult {
        let mtspool = &mut ctx.accounts.mts_pool;
        //检查转入token是否是pool对应的的token        
        //检查pool的状态
        if mtspool.is_locked {
            return Err(ErrorCode::PoolLocked.into());
        }
        //检查加入的层级状态是否为开启状态（on）

        //transfer tokens from the user's account to the pool's account
    }

    //用户赎回token
    pub fn redeem_tokens(ctx: Context<RedeemTokens>) -> ProgramResult {
        // todo check
        // todo transfer token from mtspool to user's accout

    }

    //修改mtspool状态
    pub fn change_mtspool_state(ctx: Context<ChangeMTSPoolState>, state: u8) -> ProgramResult {
        let mtspool = &mut ctx.accounts.mtspool;

        //todo 检查权限，仅程序所有者能修改状态
        mtspool.state = stata;
        Ok(())
    }

}

//set mts tokens
#[drive(accounts)]
pub struct setMTSTokens<'info> {
    #[account(
        init_if_needed,
        payer = payer
    )]
    pub mts_tokens: Account<'info, MTSTokens>,
    pub payer: Signer<'info>,
}

// initial MTS pool
#[derive(Accounts)]
pub struct initMTSPool<'info> {
    //seed: the pubkey of creator's account (in the future)
    #[account(init, seeds = [b"mtspool"], bump, payer = payer, space = 999)]
    pub mts_pool: Account<'info, MTSPool>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub mts_token: Account<'info>,
    #[account(mut)]
    pub minttoken_in_pool: Program<'info, Token>,
    pub system_program: Program<'info, System>,

}

#[derive(accounts)]
pub struct DepositTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

}

#[account]
pub struct MTSTokens {
    pub mtokens: vec<Pubkey>,
}

#[account]
pub struct MTSPool {
    pub MTSPools:vec<MTSPoolPara>
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct MTSPoolPara {
    //the creator of mtspool
    pub creator_account: Pubkey,
    //the mint token for this mtspool(represent SPL Token X)
    pub mint_token: Pubkey,
    //Address of token X liquidity account(token account for SPL Token X)
    pub token_x_account: Pubkey,       
    //initial token amount
    pub init_amt: f64,
    //the fall percent(to decide enable next layer)
    pub fall_pect: f8,
    //the multiple of next layer(base up layer)
    pub multiple: f16,
    //add uint
    pub add_unit: f8,
    //Profit target
    pub goal_amt: f64,
    //the share percent for profit
    pub prof_share_pect: f8,
    //the state of mtspool
    pub pool_status: u6,
    //pool token account to receive the withdrawal fees
    pub pool_fee_account: Pubkey,
    // the fee information
    pub fees: Fees,
    // layer data for the mtspool
    pub layer_data:vec<MTSPoolLayerData>
    // the amount sumarrize of token X
    pub token_amount_sum: f64, 
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct MTSPoolLayerData {
    //mtspool layer
    pub layer_no: u32,
    //the volum of current layer 
    pub layer_vol: f64
    //the tokens amount of current layer
    pub curl_qty: f64,
    //the users account in current layer
    layer_usrs: Vec<MTSPoolUserData>,
    //layer status
    layer_status: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct MTSPoolUserData {
    //user account in the mtspool
    pub user_account: Pubkey,
    //user token amount in the mtspool
    pub amount: f64, 
    // price when user deposited it
    pub in_price: f32,
}

#[error_code]
pub enum MTSError {
    MinTokenNotPermitedForMTSPool,
}