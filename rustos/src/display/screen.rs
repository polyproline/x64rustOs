use lazy_static::lazy_static;
use spin::RwLock;

use core::fmt::{Write,self};
use core::ptr::{write_volatile,read_volatile,NonNull};

use super::ascii::{FONT_ASCII,XCharSize,YCharSize,display_char};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenColor(u32);
impl ScreenColor{
	pub fn new(resue:u8,r:u8,g:u8,b:u8)->Self{
		Self(u32::from_ne_bytes([resue,r,g,b]))
	}
}
pub const SCREEN_HEIGHT: usize = 900;
pub const SCREEN_WIDTH: usize = 1440;

#[repr(transparent)]
pub struct ScreenBuffer(
	NonNull<[[ScreenColor; SCREEN_WIDTH]; SCREEN_HEIGHT]>,
);

unsafe impl Sync for ScreenBuffer {}
unsafe impl Send for ScreenBuffer {}

lazy_static! {
    pub static ref Screen:RwLock<ScreenBuffer> = RwLock::new(unsafe{ScreenBuffer(
		NonNull::new_unchecked(0xffff800000a00000 as *mut _)
	)});
}
pub struct PutChar<'a>{
	xPosition:usize,
	yPosition:usize,

	buffer:&'a mut ScreenBuffer,
	
	fontC:ScreenColor,
	bkC:Option<ScreenColor>,
}

impl<'a>   PutChar<'a>{
	pub fn new(buffer:&'a mut ScreenBuffer,x:usize,y:usize,fontc:ScreenColor)->PutChar{
		Self{
			xPosition:x,
			yPosition:y,
			buffer:buffer,
			fontC:fontc,
			bkC:None
		}
	}
	pub fn set_fontc(&mut self,c:ScreenColor){
		self.fontC = c;
	}
	pub fn set_bkc(&mut self,c:Option<ScreenColor>){
		self.bkC = c;
	}
	fn putchar(&mut self,c:u8){
		let fontp = unsafe{FONT_ASCII.get_unchecked(c as usize)};
		for (i,val) in fontp.iter().enumerate(){
			let mut testval = 0x80;
			unsafe{
			let buff_line = self.buffer.0.as_mut().get_unchecked_mut(self.yPosition + i);
				for j in 0..XCharSize{
					if (val & testval) != 0{
						write_volatile(buff_line.get_unchecked_mut(self.xPosition+j) as &mut _,self.fontC);
					}else{
						if let Some(c) = self.bkC{
							write_volatile(buff_line.get_unchecked_mut(self.xPosition+j) as &mut _,c);
						}
					}
					testval >>= 1;
				}
			}
		}
	}
	fn puts(&mut self,s:&str){
		let mut line = 0;
		let mut count = 0;
		let s = s.as_bytes();
		while count < s.len()|| line > 0{
			if line > 0{
				count -= 1;
				line -= 1;
				self.putchar(b' ' - b' ');
				self.xPosition += XCharSize;
				if self.xPosition >= SCREEN_WIDTH{
					self.xPosition = 0;
					self.yPosition += YCharSize;
					if self.yPosition >= SCREEN_HEIGHT{
						self.yPosition = 0;
					}
				}
			}else{
				let c = unsafe{s.get_unchecked(count).clone()};
				if c == b'\n'{
					self.xPosition = 0;
					self.yPosition += YCharSize;
					if self.yPosition >= SCREEN_HEIGHT{
						self.yPosition = 0;
					}
				}else if c == 0x08{
					if let Some(x) = self.xPosition.checked_sub(XCharSize){
						x
				   }else{
					   self.yPosition =
					   if let Some(y) = self.yPosition.checked_sub(YCharSize){
							y
					   }else{
						   (SCREEN_HEIGHT / YCharSize -1)*YCharSize
					   };
					   (SCREEN_WIDTH/ XCharSize -1)*XCharSize
				   };
				   self.putchar(b' ' - b' ');
				}else{
					if c == b'\t'{
						todo!();
					}else{
						self.putchar(display_char(c));
					}
					self.xPosition += XCharSize;
					if self.xPosition >= SCREEN_WIDTH{
						self.xPosition = 0;
						self.yPosition += YCharSize;
						if self.yPosition >= SCREEN_HEIGHT{
							self.yPosition = 0;
						}
					}
				}
			}
		}
	}
}
impl<'a> Write for PutChar<'a>{
	fn write_str(&mut self, s: &str) -> fmt::Result {
        self.puts(s);
        Ok(())
    }
}