use crate::Emulator;

pub enum OpReg {
	Opecode(u8),
	RegIndex(u8),
}

pub enum DispKind {
	Disp8(i8),
	Disp32(u32),
}

pub(crate) struct Modrm {
	pub m: u8, // Because mod is a reserved word
	// opreg: Opreg,
	pub opreg: u8,
	pub rm: u8,
	pub sib: u8,
	pub disp: DispKind,
}

impl Modrm {
	pub(crate) fn parse(emu: &mut Emulator) -> Self {
		let code = emu.get_code8(0);

		emu.eip += 1;

		let m = (code & 0xC0) >> 6;
		let opreg: u8 = (code & 0x38) >> 3;
		let rm = code & 0x07;

		let sib = if m != 0b11 && rm == 0b100 {
			let sib = emu.get_code8(0);

			emu.eip += 1;
			sib
		} else {
			0
		};

		let disp = if (m == 0b0 && rm == 0b101) || m == 0x10 {
			let disp32 = emu.get_code32(0);

			emu.eip += 4;
			DispKind::Disp32(disp32)
		} else {
			let disp8 = emu.get_sign_code8(0);

			emu.eip += 1;
			DispKind::Disp8(disp8)
		};

		Modrm {
			m,
			opreg,
			rm,
			sib,
			disp,
		}
	}
}
