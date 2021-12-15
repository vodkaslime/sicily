/* 
 * This file is part of the Sicily distribution (https://github.com/JeepYiheihou/sicily).
 * Copyright (c) 2021 Jiachen Bai.
 * 
 * This program is free software: you can redistribute it and/or modify  
 * it under the terms of the GNU General Public License as published by  
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but 
 * WITHOUT ANY WARRANTY; without even the implied warranty of 
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU 
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License 
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

/* Networking part. */
pub const PORT: u16 = 8820;
pub const OUTPUT_BUFFER_SIZE: usize = 1024;
pub const STABILIZE_FREQUENCY: u64 = 1000;

/* Algorithm part. */
pub const ID_BITS: u8 = 32;
pub const VIRTUAL_NODE_NUMBER: u8 = 8;