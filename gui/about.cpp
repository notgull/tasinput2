/*
 * src/lib.rs
 *
 * tasinput2 - Input plugin for generating tool assisted speedruns
 * Copyright (C) 2020 not_a_seagull
 *
 * This file is part of tasinput2.
 *
 * tasinput2 is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * tasinput2 is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with tasinput2.  If not, see <https://www.gnu.org/licenses/>.
 */

#include <wx/wx.h>

void ShowAboutDialog(void *parent) {
//  wxDialog *dummy = new wxDialog();
//  dummy->AssociateHandle(parent);

  wxMessageBox(
    "\ntasinput2 v1.0.0 by not_a_seagull\nBased on Direct Input by Def, modifications by Nitsuja and not_a_seagull",
    "About",
    wxOK | wxICON_INFORMATION
//    dummy
  );
}
