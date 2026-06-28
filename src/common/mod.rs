// mod.rs : src/common

macro_rules! declare_and_publish {
    ($mod_name:ident, $($type_name:ident),* $(,)?) => {
        mod $mod_name;

        pub use $mod_name::{
            $($type_name),*
        };
    };
}

declare_and_publish!(tool_version, ToolVersion);
declare_and_publish!(version_error, VersionError);


// ///////////////////////////// end of file //////////////////////////// //
