impl From<wasmtime::Error> for SettingsError {
    fn from(value: wasmtime::Error) -> Self {
        SettingsError::WasmError(value)
    }
}

pub enum SettingsError  {
    WasmError(wasmtime::Error)
}

pub enum SettingsTypeFieldPlatform {
    String(Option<String>),
    Number(Option<i32>),
    Float(Option<f32>)
}

pub struct SettingsFieldPlatform {
    pub name: String,
    pub description: String,
    pub type_field: SettingsTypeFieldPlatform
}



pub trait SettingsPlatform{
    async fn get_settings_dynamic(&mut self) -> Result<Vec<SettingsFieldPlatform>, SettingsError>;
    async fn get_settings_static() -> Result<Vec<SettingsFieldPlatform>, SettingsError>;
    async fn set_field_static(&mut self, field: SettingsFieldPlatform, value: SettingsTypeFieldPlatform) -> Result<(), SettingsError>;
    async fn set_field_dynamic(&mut self, field: SettingsFieldPlatform, value: SettingsTypeFieldPlatform) -> Result<(), SettingsError>;
}

