use witlot_platform_host::{
    WasmPlatform, exports::platform::{setting::default_setting::{FieldType, Manifest}}
    };
use aggregates::settings::{SettingsPlatform, SettingsFieldPlatform, SettingsTypeFieldPlatform, SettingsError};
mod wasm_utils;
mod aggregates;

#[derive(Debug)]
pub enum PlatformError  {
    WasmError(wasmtime::Error)
}

impl From<wasmtime::Error> for PlatformError {
    fn from(value: wasmtime::Error) -> Self {
        PlatformError::WasmError(value)
    }
}


pub struct Platform{
    pub name: String,
    pub description: String,
    pub version: String,
    settings_static: Vec<SettingsFieldPlatform>,
    wasm: WasmPlatform
}

impl Platform {
    pub async fn new(mut wasm_platform: WasmPlatform) -> Result<Self, PlatformError>{
        let settings_static = Platform::load_settings_dynamic(&mut wasm_platform).await?;
        let manifest = Platform::load_manifest(&mut wasm_platform).await?;
        Ok(       
            Platform  {
                    name: manifest.name,
                    description: manifest.description,
                    version: manifest.version,
                    settings_static,
                    wasm: wasm_platform
                }
            )
    }


    async fn load_manifest(wasm_platform: &mut WasmPlatform) -> Result<Manifest, PlatformError>{
        let manifest= wasm_platform.platform
            .platform_setting_default_setting()
            .func_get_manifest()
            .call_async(&mut wasm_platform.store, ())
            .await?
            .0;

        Ok(manifest)
    }


    async fn load_settings_dynamic(wasm_platform: &mut WasmPlatform) -> Result<Vec<SettingsFieldPlatform>, PlatformError>{
        let fields: Vec<SettingsFieldPlatform> = wasm_platform.platform
            .platform_setting_default_setting()
            .func_get_settings_static()
            .call_async(&mut wasm_platform.store, ())
            .await?
            .0
            .into_iter()
            .map(|f|{
                let type_field = match f.typesfield{
                    FieldType::Str(strr) => SettingsTypeFieldPlatform::String(strr),
                    FieldType::Number(number) => SettingsTypeFieldPlatform::Number(number),
                    FieldType::Float(number_f) => SettingsTypeFieldPlatform::Float(number_f)
                };
                SettingsFieldPlatform{
                    name: f.name,
                    description: f.description,
                    type_field
                }
            })
            .collect();
        Ok(fields)
    }
}

impl SettingsPlatform for Platform {
    async fn get_settings_dynamic(&mut self) -> Result<Vec<SettingsFieldPlatform>, SettingsError>{
        let fields: Vec<SettingsFieldPlatform> = self.wasm.platform
            .platform_setting_default_setting()
            .func_get_settings_dynamic()
            .call_async(&mut self.wasm.store, ())
            .await?
            .0
            .into_iter()
            .map(|f|{
                let type_field = match f.typesfield{
                    FieldType::Str(strr) => SettingsTypeFieldPlatform::String(strr),
                    FieldType::Number(number) => SettingsTypeFieldPlatform::Number(number),
                    FieldType::Float(number_f) => SettingsTypeFieldPlatform::Float(number_f)
                };
                SettingsFieldPlatform{
                    name: f.name,
                    description: f.description,
                    type_field
                }
            })
            .collect();
        Ok(fields)
        }

    async fn get_settings_static() -> Result<Vec<SettingsFieldPlatform>, aggregates::settings::SettingsError> {
        Ok(vec![])
    }

    async fn set_field_dynamic(&mut self, field: SettingsFieldPlatform, value: SettingsTypeFieldPlatform) -> Result<(), SettingsError> {
        Ok(())
    }

    async fn set_field_static(&mut self, field: SettingsFieldPlatform, value: SettingsTypeFieldPlatform) -> Result<(), SettingsError> {
        Ok(())
    }
}