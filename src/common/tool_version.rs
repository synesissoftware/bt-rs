// tool_version.rs : src/common

use std::cmp::Ordering;


/// Semantic version components parsed from a tool's `--version` output.
#[derive(Clone)]
#[derive(Debug)]
pub struct ToolVersion {
    pub major : u32,
    pub minor : u32,
    pub patch : u32,
    pub build : String,
}


// API functions

impl ToolVersion {
}


// Mutating methods

impl ToolVersion {
}


// Non-mutating methods

impl ToolVersion {
    fn cmp_components(
        &self,
        major : u32,
        minor : u32,
        patch : u32,
        build : &str,
    ) -> Ordering {
        self.major
            .cmp(&major)
            .then_with(|| self.minor.cmp(&minor))
            .then_with(|| self.patch.cmp(&patch))
            .then_with(|| self.build.as_str().cmp(build))
    }
}


// Implementation

impl ToolVersion {
}


// Trait implementations

impl Eq for ToolVersion {
}

impl PartialEq for ToolVersion {
    fn eq(
        &self,
        other : &Self,
    ) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialEq<(u32,)> for ToolVersion {
    fn eq(
        &self,
        other : &(u32,),
    ) -> bool {
        self.cmp_components(other.0, 0, 0, "") == Ordering::Equal
    }
}

impl PartialEq<(u32, u32)> for ToolVersion {
    fn eq(
        &self,
        other : &(u32, u32),
    ) -> bool {
        self.cmp_components(other.0, other.1, 0, "") == Ordering::Equal
    }
}

impl PartialEq<(u32, u32, u32)> for ToolVersion {
    fn eq(
        &self,
        other : &(u32, u32, u32),
    ) -> bool {
        self.cmp_components(other.0, other.1, other.2, "") == Ordering::Equal
    }
}

impl PartialEq<(u32, u32, u32, &str)> for ToolVersion {
    fn eq(
        &self,
        other : &(u32, u32, u32, &str),
    ) -> bool {
        self.cmp_components(other.0, other.1, other.2, other.3) == Ordering::Equal
    }
}

impl Ord for ToolVersion {
    fn cmp(
        &self,
        other : &Self,
    ) -> Ordering {
        self.cmp_components(other.major, other.minor, other.patch, other.build.as_str())
    }
}

impl PartialOrd for ToolVersion {
    fn partial_cmp(
        &self,
        other : &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<(u32,)> for ToolVersion {
    fn partial_cmp(
        &self,
        other : &(u32,),
    ) -> Option<Ordering> {
        Some(self.cmp_components(other.0, 0, 0, ""))
    }
}

impl PartialOrd<(u32, u32)> for ToolVersion {
    fn partial_cmp(
        &self,
        other : &(u32, u32),
    ) -> Option<Ordering> {
        Some(self.cmp_components(other.0, other.1, 0, ""))
    }
}

impl PartialOrd<(u32, u32, u32)> for ToolVersion {
    fn partial_cmp(
        &self,
        other : &(u32, u32, u32),
    ) -> Option<Ordering> {
        Some(self.cmp_components(other.0, other.1, other.2, ""))
    }
}

impl PartialOrd<(u32, u32, u32, &str)> for ToolVersion {
    fn partial_cmp(
        &self,
        other : &(u32, u32, u32, &str),
    ) -> Option<Ordering> {
        Some(self.cmp_components(other.0, other.1, other.2, other.3))
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::ToolVersion;

    use std::cmp::Ordering;


    #[test]
    fn TEST_ToolVersion_Ord_AGAINST_ToolVersion() {
        let a = ToolVersion {
            major : 1,
            minor : 94,
            patch : 0,
            build : String::new(),
        };
        let b = ToolVersion {
            major : 1,
            minor : 94,
            patch : 1,
            build : String::new(),
        };

        assert_eq!(a.cmp(&b), Ordering::Less);
        assert!(a < b);
    }

    #[test]
    fn TEST_ToolVersion_PartialOrd_AGAINST_major() {
        let version = ToolVersion {
            major : 2,
            minor : 0,
            patch : 0,
            build : String::new(),
        };

        assert!(version > (1,));
        assert!(version >= (2,));
        assert_eq!(version.partial_cmp(&(2,)), Some(Ordering::Equal));
    }

    #[test]
    fn TEST_ToolVersion_PartialOrd_AGAINST_major_minor() {
        let version = ToolVersion {
            major : 1,
            minor : 94,
            patch : 0,
            build : String::new(),
        };

        assert!(version >= (1, 94));
        assert!(version >= (1, 93));
        assert!(version < (1, 95));
        assert!(version < (2, 0));
    }

    #[test]
    fn TEST_ToolVersion_PartialOrd_AGAINST_major_minor_patch() {
        let version = ToolVersion {
            major : 1,
            minor : 96,
            patch : 0,
            build : String::new(),
        };

        assert!(version >= (1, 96, 0));
        assert!(version < (1, 96, 1));
    }

    #[test]
    fn TEST_ToolVersion_PartialOrd_AGAINST_major_minor_patch_build() {
        let version = ToolVersion {
            major : 1,
            minor : 96,
            patch : 0,
            build : "abc".to_string(),
        };

        assert!(version >= (1, 96, 0, "abc"));
        assert!(version < (1, 96, 0, "abd"));
    }
}


// ///////////////////////////// end of file //////////////////////////// //
