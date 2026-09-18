//! Genesis Engine SDK facade for runtime, editor, assets and plugins.
pub trait Plugin{fn name(&self)->&'static str;}
pub trait Asset{fn asset_id(&self)->&str;}
pub trait Runtime{fn start(&mut self)->Result<(),String>;fn stop(&mut self)->Result<(),String>;}
pub trait Editor{fn open(&mut self)->Result<(),String>;fn close(&mut self)->Result<(),String>;}
