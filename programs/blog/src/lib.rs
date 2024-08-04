use anchor_lang::prelude::*;

declare_id!("Ex4CxjdqdvXsfcGFwgkgnp2fUsxYLFwPRJMoRmQ52B3W");

#[program]
pub mod blog {
    use super::*;

    pub fn init_blog(ctx: Context<Initialize>) -> Result<()> {
        let blog = &mut ctx.accounts.blog;
        blog.authority = *ctx.accounts.authority.key;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, seeds = [b"blog", authority.key().as_ref()], bump, payer = authority, space = 8 + size_of::<Blog>())]
    pub blog: Account<'info, Blog>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String, content: String)]
pub struct CreatePost<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut, has_one = authority)]
    pub blog: Account<'info, Blog>,

    #[account(init, seeds = [b"post", blog.key().as_ref(), &blog.posts.to_be_bytes()], bump, payer = authority, 
        space = 8 + size_of::<Post>() + title.as_bytes().len() + content.as_bytes().len() - 40)]
    pub post: Account<'info, Post>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct Blog {
    authority: Pubkey,
    latest: Pubkey,
    posts: u64,
}

#[account]
#[derive(Default)]
pub struct Post {
    title: String,
    content: String,
    timestamp: i64,
    blog: Pubkey,
    previous: Pubkey,
}
