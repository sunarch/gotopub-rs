/*
GoToPub - GoToSocial and general Fediverse client
Copyright (C) 2024  András Németh

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

pub const DISPLAY_NAME: &str = "GoToPub";

// The full version of the package.
pub const PKG_VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

// The major version of the package.
pub const PKG_VERSION_MAJOR: Option<&str> = option_env!("CARGO_PKG_VERSION_MAJOR");

// The minor version of the package.
pub const PKG_VERSION_MINOR: Option<&str> = option_env!("CARGO_PKG_VERSION_MINOR");

// The patch version of the package.
pub const PKG_VERSION_PATCH: Option<&str> = option_env!("CARGO_PKG_VERSION_PATCH");

// The pre-release version of the package.
pub const PKG_VERSION_PRE: Option<&str> = option_env!("CARGO_PKG_VERSION_PRE");

// Colon separated list of authors from the manifest of your package.
pub const PKG_AUTHORS: Option<&str> = option_env!("CARGO_PKG_AUTHORS");

// The name of the package.
pub const PKG_NAME: Option<&str> = option_env!("CARGO_PKG_NAME");

// The description from the manifest of the package.
pub const PKG_DESCRIPTION: Option<&str> = option_env!("CARGO_PKG_DESCRIPTION");

// The home page from the manifest of the package.
pub const PKG_HOMEPAGE: Option<&str> = option_env!("CARGO_PKG_HOMEPAGE");

// The repository from the manifest of the package.
pub const PKG_REPOSITORY: Option<&str> = option_env!("CARGO_PKG_REPOSITORY");

// The license from the manifest of the package.
pub const PKG_LICENSE: Option<&str> = option_env!("CARGO_PKG_LICENSE");

// The license file from the manifest of the package.
pub const PKG_LICENSE_FILE: Option<&str> = option_env!("CARGO_PKG_LICENSE_FILE");

// The minimum supported Rust version from the manifest of the package.
pub const PKG_RUST_VERSION: Option<&str> = option_env!("CARGO_PKG_RUST_VERSION");

// Path to the README file of the package.
pub const PKG_README: Option<&str> = option_env!("CARGO_PKG_README");
