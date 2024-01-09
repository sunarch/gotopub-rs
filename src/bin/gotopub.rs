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

use gotopub::meta;

fn main() {
    let pkg_name: &str = meta::PKG_NAME.unwrap_or("(unknown name)");
    let pkg_version: &str = meta::PKG_VERSION.unwrap_or("UNKNOWN");

    println!("{} - {} v{}", meta::DISPLAY_NAME, pkg_name, pkg_version);
}
