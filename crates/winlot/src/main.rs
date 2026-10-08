use witlot_platform_host::get_wasm_platforms;
use witlot_platform::{Platform, PlatformError};

#[derive(Debug)]
enum ErrorApllication{
   PlatformError(PlatformError)
}

impl From<PlatformError> for ErrorApllication {
    fn from(value: PlatformError) -> Self {
        Self::PlatformError(value)
    }
}

#[tokio::main]
async fn main() -> Result<(), ErrorApllication>{
    let mut plaforms = Vec::new();
    for wasm_platform in  get_wasm_platforms().await{
        plaforms.push(Platform::new(wasm_platform).await?);
    }
    Ok(())
}   
