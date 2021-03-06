/* kernel::sgash.rs */

use core::*;
use core::str::*;
use core::option::{Some, Option, None}; // Match statement
use core::iter::Iterator;
use kernel::*;
use super::super::platform::*;
use kernel::memory::Allocator;


pub static mut buffer: cstr = cstr {
				p: 0 as *mut u8,
				p_cstr_i: 0,
				max: 0
			      };

pub static mut winlist: windowlist = windowlist {
					head: 0 as *mut windownode
				};

pub static mut root: *mut dirnode = 0 as *mut dirnode;

pub static mut wd: *mut dirnode = 0 as *mut dirnode;

pub static mut tbar: taskbar = taskbar {
					menu_open: false
				};

pub static mut arr1: bool = false;

pub static mut arr2: bool = false;

pub static mut win_count: uint = 0;

pub fn putchar(key: char) {
    unsafe {
	/*
	 * We need to include a blank asm call to prevent rustc
	 * from optimizing this part out
	 */
	asm!("");
	io::write_char(key, io::UART0);
    }
}

fn putstr(msg: &str) {
    for c in slice::iter(as_bytes(msg)) {
	putchar(*c as char);
    }
}

pub unsafe fn drawstr_at_coord(msg: &str, x: u32, y: u32, color: u32) {
    let mut i = 0;
    for c in slice::iter(as_bytes(msg)) {
	drawchar_at_coord(*c as char, x+i*io::CURSOR_WIDTH, y, color);
	i += 1;
    }
}

pub unsafe fn putcstr(s: cstr)
{
    let mut p = s.p as uint;
    while *(p as *char) != '\0'
    {	
	putchar(*(p as *char));
	p += 1;
    }
}

pub unsafe fn drawcstr_at_coord(s: cstr, x: u32, y: u32, color: u32)
{
	let mut p = s.p as uint;
	let mut i = 0;
    while *(p as *char) != '\0'
    {	
	drawchar_at_coord(*(p as *char), x+i*io::CURSOR_WIDTH, y, color);
	p += 1;
	i += 1;
    }
}

pub unsafe fn drawcstr_in_window(s: cstr, x: u32, y: u32, width: u32, color: u32)
{
	let mut p = s.p as uint;
	let mut i = 0;
	let mut j = 0;
    while *(p as *char) != '\0'
    {	
	if (((i+1)*io::CURSOR_WIDTH) > width) {
		i = 0;
		j += 1;
	}
	drawchar_at_coord(*(p as *char), x+i*io::CURSOR_WIDTH, y+j*io::CURSOR_HEIGHT, color);
	p += 1;
	i += 1;
    }
}

pub unsafe fn parsekey(x: char) {
	let x = x as u8;
	// Set this to false to learn the keycodes of various keys!
	// Key codes are printed backwards because life is hard	
		match x { 
			13		=>	{ 
						parse();
						prompt(false); 
			}
			127		=>	{ 
				if (buffer.delete_char()) { 
					putchar('');
					putchar(' ');
					putchar(''); 
				}
			}
			9		=>	{
				let win = winlist.get_bot();
				bring_window_to_top(win.id);
			}
			92		=>	{
				if (!tbar.menu_open && cursor_in(0, 449, 57, 29)) {
					tbar.open_menu();
				}
				else if (cursor_in(0, 449, 57, 29)) {
					tbar.close_menu();
					tbar.clear_menu();
				}
				else if (tbar.menu_open && cursor_in(0, 388, 100, 60))
				{
					if (cursor_in(0, 388, 100, 40)) {
						if (cursor_in(0, 388, 100, 20)) {
							//putcstr((*wd).di.dname);
						}
						else {
							winlist.delete_all();
							win_count = 0;
						}
					}
					else {					
						create_window();
					}
				}
				else if (cursor_in(58, 449, 580, 29)) {
					let mut current = winlist.head;
					let mut i: u32 = 0;
					while ((current as uint) != 0) {
						if(cursor_in(58 + 70*i, 449, 70, 29)) {
							if ((*current).win.visible) {
								let top = winlist.get_top_win();
								if (top == current) {
									(*current).win.blank();
									(*current).win.visible = false;
								}
								else {
									bring_window_to_top((*current).win.id);
								}
							}
							else {
								(*current).win.visible = true;
								bring_window_to_top((*current).win.id);
							}
						}
						current = (*current).next;
						i += 1;
					}
				}
				else {
					let selected_window = winlist.top_window_under_cursor();
					if ((selected_window as uint) != 0) {
						if (cursor_in((*selected_window).win.x+(*selected_window).win.width-10, (*selected_window).win.y-18, 12, 12) && (*selected_window).win.visible) {
							(*selected_window).win.blank();
							winlist.remove_win((*selected_window).win.id);
							win_count -= 1;
						}
						else if (cursor_in((*selected_window).win.x+(*selected_window).win.width-26, (*selected_window).win.y-18, 12, 12) && (*selected_window).win.visible) {
							(*selected_window).win.blank();
							(*selected_window).win.visible = false;
						}
						else if (cursor_in((*selected_window).win.x-7, (*selected_window).win.y-22, (*selected_window).win.width + 14, 22) && (*selected_window).win.visible) {
							bring_window_to_top((*selected_window).win.id);
							if ((*selected_window).win.attached) {
								(*selected_window).win.attached = false;
							}
							else {
								(*selected_window).win.attached = true;
							}
						}
						else if (cursor_in((*selected_window).win.x+(*selected_window).win.width-3, (*selected_window).win.y+(*selected_window).win.height-3, 8, 8) && (*selected_window).win.visible) {
							if ((*selected_window).win.expanding) {
								(*selected_window).win.expanding = false;
							}
							else {
								(*selected_window).win.expanding = true;
							}
						}
						else {
							bring_window_to_top((*selected_window).win.id);
						}
					}
				}
			}
			0x1B	=>	{
				arr1 = true;
			}
			0x5B	=>	{
				if (arr1) {
					arr2 = true;
					arr1 = false;
				}
			}
			0x41	=>	{
				if (arr2) {
					io::blank_cursor();
					io::move_cursor_up();
					winlist.mov_attached(0);
					winlist.exp_attached(0);
					arr2 = false;
				}
			}
			0x42	=>	{
				if (arr2) {
					io::blank_cursor();
	   				io::move_cursor_down();
					winlist.mov_attached(1);
					winlist.exp_attached(1);
					arr2 = false;
				}
			}
			0x43	=>	{
				if (arr2) {
					io::blank_cursor();
	   				io::move_cursor_right();
					winlist.mov_attached(3);
					winlist.exp_attached(3);
					arr2 = false;
				}
			}
			0x44	=>	{
				if (arr2) {
					io::blank_cursor();
	   				io::move_cursor_left();
					winlist.mov_attached(2);
					winlist.exp_attached(2);
					arr2 = false;
				}
			}
			_		=>	{
				if (buffer.add_char(x)) { 
					putchar(x as char);
				}
			}
		}
		paint();
}

unsafe fn create_window() {
	win_count += 1;
	let mut w = window::new(cstr::from_str("Window"), 50 + 10*(win_count as u32), 50 + 10*(win_count as u32), 200, 200, win_count, true);
	winlist.add_win_back(w);
}

unsafe fn drawchar_at_coord(c: char, x: u32, y: u32, color: u32)
{    
    io::draw_char_at(c, x, y, color);
}

pub unsafe fn init() {
    buffer = cstr::new(256);
    let root_name = cstr::from_str(&"root");
    let root_dir = directory::new(root_name, 0 as *mut dirnode);
    //winlist = windowlist::new();
    //tbar = taskbar::new();
    tbar.draw();
    root = dirnode::new(root_dir);
    wd = root;
    prompt(true);
}

unsafe fn prompt(startup: bool) {
	putstr(&"\nsgash> ");
	//if !startup {drawstr(&"\nsgash> ");}
	buffer.reset();
}

unsafe fn parse() {
		match buffer.getarg(' ', 0) {
		    Some(y)        => {
			if(y.streq(&"mv")) {
			    match buffer.getarg(' ', 1) {
				Some(filename)        => {
				    match buffer.getarg(' ', 2) {
					Some(dir)        => {
						let mut subdir = (*wd).di.get_child(dir);
				    		if ((subdir as uint) != 0) {
							(*subdir).di.add_file((*wd).di.get_file(filename));
							(*wd).di.files.remove_file_nofree(filename);
						}
						else {
							(*wd).di.files.set_new_name(filename, dir);
						}
					}
					None        => { }
				    };
				}
				None        => { }
			    };
			}
			else if(y.streq(&"rm")) {
			    match buffer.getarg(' ', 1) {
				Some(x)        => {
				    delete_file(wd, x);
				    delete_directory((*wd).di.get_child(x));
				}
				None        => { }
			    };
			}
			else if(y.streq(&"cd")) {
			    let (cmd,dir) = buffer.splitonce(' ');
			    if (dir.streq("")) {
				wd = root;
			    }
			    else {
			    	let subdir = (*wd).di.get_child(dir);
				if !(((subdir as uint) == 0) || ((subdir as u32) == io::BG_COLOR)) {
					wd = subdir;
				}
			    }
			}
			else if(y.streq(&"mkdir")) {
			    let (cmd, dir) = buffer.splitonce(' ');
			    create_directory(wd, dir);
			}
			else if(y.streq(&"wr")) {
			    match buffer.getarg(' ', 1) {
				Some(filename)        => {
				    match buffer.getarg(' ', 2) {
					Some(words)        => {
						let mut file = (*wd).di.get_file(filename);
				    		if !(file.fname.streq(&"")) {
							(*wd).di.write_file(filename, words);
						}
						else {
							create_file(wd, filename);
							(*wd).di.write_file(filename, words);
						}
					}
					None        => { }
				    };
				}
				None        => { }
			    };
			}
		    }
		    None        => { }
		};
	buffer.reset();
}

unsafe fn paint() {
	winlist.draw_all();
	tbar.draw();
	io::draw_cursor();
}

unsafe fn cursor_in(x: u32, y: u32, width: u32, height: u32) -> bool {
	(io::CURSOR_X > x) && (io::CURSOR_X < (x + width)) && (io::CURSOR_Y > y) && (io::CURSOR_Y < (y + height))
}

unsafe fn read_file(filename: cstr) -> cstr {
	(*wd).di.read_file(filename)
}

//unsafe fn write_file(filename: cstr, word: cstr) -> bool {
//	(*wd).di.write_file(filename, word)
//}

unsafe fn create_file(dir: *mut dirnode, name: cstr) {
	let mut newfile = file::new(1024, name);
	(*dir).di.add_file(newfile);
}

unsafe fn delete_file(dir: *mut dirnode, name: cstr) -> bool {
	(*dir).di.remove_file(name)
}

//unsafe fn list_directory(dir: *mut dirnode) {
//	(*dir).di.directory_file_list()
//}

unsafe fn create_directory(parent: *mut dirnode, dnm: cstr) {
	let mut newdir = directory::new(dnm, parent);
	(*parent).di.add_dir(newdir);
}

unsafe fn delete_directory(dir: *mut dirnode) -> bool {
	if ((*dir).di.empty()) {
		heap.free((*dir).di.files.head as *mut u8);
		(*dir).di.dname.destroy();
		(*(*dir).di.parent).di.remove_dir((*dir).di.dname);
		return true;
	}
	false
}

unsafe fn get_directory(parent: *mut dirnode, dnm: cstr) -> *mut dirnode {
	(*parent).di.get_child(dnm)
}


/* BUFFER MODIFICATION FUNCTIONS */

struct directory {
	parent: *mut dirnode,
	child_dir: dirlist,
	files: filelist,
	dname: cstr
}

impl directory {
	pub unsafe fn new(name: cstr, myparent: *mut dirnode) -> directory {
		let this = directory {
			parent: myparent,
			child_dir: dirlist::new(),
			files: filelist::new(),
			dname: name
		};
		this
	}

	pub unsafe fn add_file(&mut self, f: file) {
		self.files.add_file(f);
	}

	pub unsafe fn remove_file(&mut self, fname: cstr) -> bool {
		self.files.remove_file(fname)
	}

	pub unsafe fn add_dir(&mut self, d: directory) {
		self.child_dir.add_dir(d);
	}

	pub unsafe fn remove_dir(&mut self, dname: cstr) -> bool {
		self.child_dir.remove_dir(dname)
	}

	pub unsafe fn get_parent(&mut self) -> *mut dirnode {
		self.parent
	}

	pub unsafe fn get_child(&mut self, dnm: cstr) -> *mut dirnode {
		self.child_dir.get_dirnode(dnm)
	}

	pub unsafe fn read_file(&mut self, fnm: cstr) -> cstr {
		self.files.read_file(fnm)
	}

	pub unsafe fn write_file(&mut self, fnm: cstr, word: cstr) -> bool {
		self.files.write_file(fnm, word)
	}

	pub unsafe fn get_file(&mut self, fnm: cstr) -> file {
		self.files.get_file(fnm)
	}

	pub unsafe fn directory_file_list(&mut self) {
		self.files.print_filenames();
		self.child_dir.print_dirnames();
	}

	pub unsafe fn empty(&mut self) -> bool {
		(self.files.empty()) && (self.child_dir.empty())
	}

}

struct dirnode {
	next: *mut dirnode,
	di: directory
}

impl dirnode {
	pub unsafe fn new(d: directory) -> *mut dirnode {
		let (x, y) = heap.alloc(64);
		let this = dirnode{
			next: 0 as *mut dirnode,
			di: d
		};
		*(x as *mut dirnode) = this;
		x as *mut dirnode
	}
}

struct dirlist {
	head: *mut dirnode
}

impl dirlist {
	pub unsafe fn new() -> dirlist {		
		let this = dirlist {
			head: 0 as *mut dirnode
		};
		this
	}

	pub unsafe fn add_dir(&mut self, d: directory) {
		let mut current = self.head;
		if (((current as uint) == 0) || ((current as u32) == io::BG_COLOR)) {
			self.head = dirnode::new(d);
		}
		else {
			while !((((*current).next as uint) == 0) || (((*current).next as u32) == io::BG_COLOR)) {
				current = (*current).next;
			}
			(*current).next = dirnode::new(d);
		}
	}

	pub unsafe fn remove_dir(&mut self, dnm: cstr) -> bool {
		let mut current = self.head;
		if ((current as uint) == 0) {
			return false;
		}
		if (((*current).di.dname).eq(&dnm)) {
			let temp = (*current).next;
			heap.free(current as *mut u8);
			self.head = temp;
			return true;
		};
		while !((((*current).next as uint) == 0) || (((*current).next as u32) == io::BG_COLOR)) {
			if (((*(*current).next).di.dname).eq(&dnm)) {
				let temp = (*(*current).next).next;
				heap.free((*current).next as *mut u8);
				(*current).next = temp;
				return true;
			};
			current = (*current).next;
		}
		false
	}

	pub unsafe fn get_dirnode(&mut self, dnm: cstr) -> *mut dirnode {
		let mut current = self.head;
		while !(((current as uint) == 0) || ((current as u32) == io::BG_COLOR)) {
			if (((*current).di.dname).eq(&dnm)) {
				return current;
			};
			current = (*current).next;
		}
		return 0 as *mut dirnode;
	}

	pub unsafe fn print_dirnames(&mut self) {
		let mut current = self.head;
		while !(((current as uint) == 0) || ((current as u32) == io::BG_COLOR)) {
			putstr("\n");
			putcstr((*current).di.dname);
			current = (*current).next;
		}
	}

	pub unsafe fn empty(&mut self) -> bool {
		(((self.head as uint) == 0) || ((self.head as u32) == io::BG_COLOR))
	}
}

struct filenode {
	next: *mut filenode,
	fi: file
}

impl filenode {
	pub unsafe fn new(f: file) -> *mut filenode {
		let (x, y) = heap.alloc(64);
		let this = filenode{
			next: 0 as *mut filenode,
			fi: f
		};
		*(x as *mut filenode) = this;
		x as *mut filenode
	}
}

struct filelist {
	head: *mut filenode
}

impl filelist {
	pub unsafe fn new() -> filelist {
		let name = cstr::from_str(&"");
		let headfile = file {
			fname: name,
			contents: name
		};
		let this = filelist{
			head: filenode::new(headfile)
		};
		this
	}

	pub unsafe fn add_file(&mut self, f: file) {
		let mut current = self.head;
		while !(((((*current).next) as uint) == 0) || ((((*current).next) as u32) == io::BG_COLOR)) {
			current = (*current).next;
		}
		(*current).next = filenode::new(f);
	}

	pub unsafe fn remove_file(&mut self, fnm: cstr) -> bool {
		let mut current = self.head;
		while !(((((*current).next) as uint) == 0) || ((((*current).next) as u32) == io::BG_COLOR)) {
			if (((*(*current).next).fi.fname).eq(&fnm)) {
				let temp = (*(*current).next).next;
				heap.free((*current).next as *mut u8);
				(*current).next = temp;
				return true;
			};
			current = (*current).next;
		}
		false
	}

	pub unsafe fn remove_file_nofree(&mut self, fnm: cstr) -> bool {
		let mut current = self.head;
		while !(((((*current).next) as uint) == 0) || ((((*current).next) as u32) == io::BG_COLOR)) {
			if (((*(*current).next).fi.fname).eq(&fnm)) {
				let temp = (*(*current).next).next;
				(*current).next = temp;
				return true;
			};
			current = (*current).next;
		}
		false
	}

	pub unsafe fn read_file(&mut self, fnm: cstr) -> cstr {
		let mut current = self.head;
		while !(((((*current).next) as uint) == 0) || ((((*current).next) as u32) == io::BG_COLOR)) {
			if (((*(*current).next).fi.fname).eq(&fnm)) {
				return (*(*current).next).fi.get_contents();
			};
			current = (*current).next;
		}
		return cstr::from_str(&"");
	}

	pub unsafe fn write_file(&mut self, fnm: cstr, word: cstr) -> bool {
		let mut current = self.head;
		while !(((((*current).next) as uint) == 0) || ((((*current).next) as u32) == io::BG_COLOR)) {
			if (((*(*current).next).fi.fname).eq(&fnm)) {
				return (*(*current).next).fi.append_cstr(word);
			};
			current = (*current).next;
		}
		return false;
	}

	pub unsafe fn get_file(&mut self, fnm: cstr) -> file {
		let mut current = self.head;
		while !(((((*current).next) as uint) == 0) || ((((*current).next) as u32) == io::BG_COLOR)){
			if (((*(*current).next).fi.fname).eq(&fnm)) {
				return (*(*current).next).fi;
			};
			current = (*current).next;
		}
		return (*self.head).fi;
	}

	pub unsafe fn print_filenames(&mut self) {
		let mut current = self.head;
		while !(((((*current).next) as uint) == 0) || ((((*current).next) as u32) == io::BG_COLOR)) {
			putstr("\n");
			putcstr((*(*current).next).fi.fname);
			current = (*current).next;
		}
	}

	pub unsafe fn set_new_name(&mut self, oldname: cstr, newname: cstr) {
		let mut current = self.head;
		while !(((((*current).next) as uint) == 0) || ((((*current).next) as u32) == io::BG_COLOR)){
			if (((*(*current).next).fi.fname).eq(&oldname)) {
				(*(*current).next).fi.fname = newname;
			};
			current = (*current).next;
		}
	}

	pub unsafe fn empty(&mut self) -> bool {
		(((((*self.head).next) as uint) == 0) || ((((*self.head).next) as u32) == io::BG_COLOR))
	}
}

struct file {
	fname: cstr,
	contents: cstr
}

impl file {
	pub unsafe fn new(size: uint, name: cstr) -> file {
		let this = file {
			fname: name,
			contents: cstr::new(size)
		};
		this
	}

	pub unsafe fn eq(&mut self, f: file) -> bool {
		(self.fname.eq(&f.fname)) && (self.contents.eq(&f.contents))
	}

	pub unsafe fn append_cstr(&mut self, s: cstr) -> bool {
		if ((self.contents.p_cstr_i + s.len()) >= self.contents.max) {return false;}
		let mut p = s.p as uint;
    		while *(p as *char) != '\0'
    		{	
			self.contents.add_char(*(p as *u8));
			p += 1;
    		}
		true
	}

	pub unsafe fn len(&mut self) -> uint {
		self.contents.p_cstr_i
	}

	pub unsafe fn get_contents(&mut self) -> cstr {
		self.contents
	}
}


struct cstr {
	p: *mut u8,
	p_cstr_i: uint,
	max: uint 
}

impl cstr {
	pub unsafe fn new(size: uint) -> cstr {
		// Sometimes this doesn't allocate enough memory and gets stuck...
		let (x, y) = heap.alloc(size);
		let this = cstr {
			p: x,
			p_cstr_i: 0,
			max: y
		};
		*(((this.p as uint)+this.p_cstr_i) as *mut char) = '\0';
		this
	}


#[allow(dead_code)]
	unsafe fn from_str(s: &str) -> cstr {
		let mut this = cstr::new(256);
		for c in slice::iter(as_bytes(s)) {
			this.add_char(*c);
		};
		this
	}

#[allow(dead_code)]
	fn len(&self) -> uint { self.p_cstr_i }

	// HELP THIS DOESN'T WORK THERE IS NO GARBAGE COLLECTION!!!
	// -- TODO: exchange_malloc, exchange_free
#[allow(dead_code)]
	unsafe fn destroy(&self) { heap.free(self.p); }

	unsafe fn add_char(&mut self, x: u8) -> bool{
		if (self.p_cstr_i == self.max) { return false; }
		*(((self.p as uint)+self.p_cstr_i) as *mut u8) = x;
		self.p_cstr_i += 1;
		*(((self.p as uint)+self.p_cstr_i) as *mut char) = '\0';
		true
	}

	unsafe fn concatfront(&mut self, word: cstr) -> bool {
		if ((self.p_cstr_i + word.len()) >= self.max) {return false;}
		let mut p = (word.p as uint) + word.p_cstr_i - 1;
    		while p >= (word.p as uint)
    		{	
			let mut i: uint = self.p_cstr_i + 1;
			while i > 0 {
				*(((self.p as uint)+i) as *mut u8) = *(((self.p as uint)+i-1) as *mut u8);
				i -= 1;
			}
			self.p_cstr_i += 1;
			*(((self.p as uint)+self.p_cstr_i) as *mut char) = '\0';
			*(self.p as *mut u8) = *(p as *mut u8);
			p -= 1;
    		}
		true
	}

	unsafe fn place_in_mem(&mut self, x: *mut u8) {
		let mut p = self.p as uint;
		let mut xp = x as uint;
   		while *(p as *char) != '\0'
    		{	
			*(xp as *mut char) = *(p as *char);
			xp += 1;
			p += 1;
    		}	
	}

	unsafe fn copy(&mut self, word: cstr) {
		self.reset();
		let mut p = word.p as uint;
   		while *(p as *char) != '\0'
    		{	
			self.add_char(*(p as *u8));
			p += 1;
    		}
	}

	unsafe fn delete_char(&mut self) -> bool {
		if (self.p_cstr_i == 0) { return false; }
		self.p_cstr_i -= 1;
		*(((self.p as uint)+self.p_cstr_i) as *mut char) = '\0';
		true
	}

	unsafe fn reset(&mut self) {
		self.p_cstr_i = 0; 
		*(self.p as *mut char) = '\0';
	}

#[allow(dead_code)]
	unsafe fn eq(&self, other: &cstr) -> bool {
		if (self.len() != other.len()) { return false; }
		else {
			let mut x = 0;
			let mut selfp: uint = self.p as uint;
			let mut otherp: uint = other.p as uint;
			while x < self.len() {
				if (*(selfp as *char) != *(otherp as *char)) { return false; }
				selfp += 1;
				otherp += 1;
				x += 1;
			}
			true
		}
	}

	unsafe fn streq(&self, other: &str) -> bool {
		let mut selfp: uint = self.p as uint;
		for c in slice::iter(as_bytes(other)) {
			if( *c != *(selfp as *u8) ) { return false; }
			selfp += 1;
		};
		*(selfp as *char) == '\0'
	}

	unsafe fn getarg(&self, delim: char, mut k: uint) -> Option<cstr> {
		let mut ind: uint = 0;
		let mut found = k == 0;
		let mut selfp: uint = self.p as uint;
		let mut s = cstr::new(256);
		loop {
			if (*(selfp as *char) == '\0') { 
				// End of string
				if (found) { return Some(s); }
				else { return None; }
			};
			if (*(selfp as *u8) == delim as u8) { 
				if (found) { return Some(s); }
				k -= 1;
			};
			if (found) {
				s.add_char(*(selfp as *u8));
			};
			found = k == 0;
			selfp += 1;
			ind += 1;
			if (ind == self.max) { 
				return None; 
			}
		}
	}

#[allow(dead_code)]
	/*unsafe fn split(&self, delim: char) -> (cstr, cstr) {
		let mut selfp: uint = self.p as uint;
		let mut beg = cstr::new(256);
		let mut end = cstr::new(256);
		let mut found = false;
		loop {
			if (*(selfp as *char) == '\0') { 
				return (beg, end);
			}
			else if (*(selfp as *u8) == delim as u8) {
				found = true;
			}
			else if (!found) {
				beg.add_char(*(selfp as *u8));
			}
			else if (found) {
				end.add_char(*(selfp as *u8));
			};
			selfp += 1;
		}
	}*/

#[allow(dead_code)]
	unsafe fn splitonce(&self, delim: char) -> (cstr, cstr) {
		let mut selfp: uint = self.p as uint;
		let mut beg = cstr::new(256);
		let mut end = cstr::new(256);
		let mut found = false;
		loop {
			if (*(selfp as *char) == '\0') { 
				return (beg, end);
			}
			else if (*(selfp as *u8) == delim as u8) {
				if (found) { end.add_char(*(selfp as *u8)); };
				found = true;
			}
			else if (!found) {
				beg.add_char(*(selfp as *u8));
			}
			else if (found) {
				end.add_char(*(selfp as *u8));
			};
			selfp += 1;
		}
	}


}


pub unsafe fn bring_window_to_top(wid: uint) {
	winlist.bring_to_top(wid);
}

struct window {
	name: cstr,
	x: u32,
	y: u32,
	width: u32,
	height: u32,
	visible: bool,
	id: uint,
	attached: bool,
	expanding: bool,
	data: cstr
}

impl window {
	pub unsafe fn new(name: cstr, x: u32, y: u32, width: u32, height: u32, id: uint, visible: bool) -> window {
		let this = window {
			name: name,
			x: x,
			y: y,
			width: width,
			height: height,
			visible: visible,
			id: id,
			attached: false,
			expanding: false,
			data: cstr::from_str("Things go here.")
		};
		this
	}

	/*pub unsafe fn cursor_within(&mut self) -> bool {
		(io::CURSOR_X > (self.x-7-io::CURSOR_WIDTH)) && (io::CURSOR_X < (self.x+self.width+12)) && (io::CURSOR_Y > (self.y-22- io::CURSOR_HEIGHT)) && (io::CURSOR_Y < (self.y+self.height+7))
	}*/

	pub unsafe fn draw(&mut self) {
		if (self.visible) {
			io::fill_box(self.x, self.y, self.width, self.height, 0x777777);
			io::draw_box(self.x-1, self.y-1, self.width+2, self.height+2,0x000000);
	    		io::draw_frame(self.x-6, self.y-6, self.width+12, self.height+12, 5, 0x666666);
			io::fill_box(self.x-6, self.y-21, self.width+12, 15, 0x666666);
			io::draw_box(self.x-7, self.y-22, self.width+14, self.height+29, 0x000000);
			io::draw_box(self.x+self.width-10, self.y-18, 12, 12, 0x000000);
			io::draw_line(self.x+self.width-10, self.y-18, self.x+self.width+2, self.y-6, 0x000000);
			io::draw_line(self.x+self.width+2, self.y-18, self.x+self.width-10, self.y-6, 0x000000);
			io::draw_box(self.x+self.width-26, self.y-18, 12, 12, 0x000000);
			io::draw_line(self.x+self.width-24, self.y-9, self.x+self.width-15, self.y-9, 0x000000);
			drawcstr_at_coord(self.name, self.x, self.y - 19, 0x000000);
			drawcstr_in_window(self.data, self.x, self.y, self.width, 0x000000);
		}
	}

	pub unsafe fn blank(&mut self) {
		io::fill_box(self.x-7, self.y-22, self.width+14, self.height+29, io::BG_COLOR);
	}
}

struct windownode {
	next: *mut windownode,
	win: window
}

impl windownode {
	pub unsafe fn new(w: window) -> *mut windownode {
		let (x, y) = heap.alloc(128);
		let this = windownode {
			next: 0 as *mut windownode,
			win: w
		};
		*(x as *mut windownode) = this;
		(x as *mut windownode)
	}
}

struct windowlist {
	head: *mut windownode
}

impl windowlist {
	pub unsafe fn new() -> windowlist {
		let this = windowlist {
			head: 0 as *mut windownode
		};
		this
	}

	pub unsafe fn get_windownode(&mut self, id: uint) -> *mut windownode {
		let mut current = self.head;
		while ((current as uint) != 0) {
			if ((*current).win.id == id) {
				return current;
			}
			current = (*current).next;
		}
		current
	}

	pub unsafe fn mov_attached(&mut self, direction: u32) {
		let mut current = self.head;
		while ((current as uint) != 0) {
			if ((*current).win.attached) {
				(*current).win.blank();
				if (direction == 0) {
					(*current).win.y = (*current).win.y - 8;
				}
				else if (direction == 1) {
					(*current).win.y = (*current).win.y + 8;
				}
				else if (direction == 2) {
					(*current).win.x = (*current).win.x - 8;
				}
				else {
					(*current).win.x = (*current).win.x + 8;
				}
			}
			current = (*current).next;
		}
	}

	/*pub unsafe fn mov_attached_up(&mut self) {
		let mut current = self.head;
		while ((current as uint) != 0) {
			if ((*current).win.attached) {
				(*current).win.blank();
				(*current).win.y = (*current).win.y - 8;
			}
			current = (*current).next;
		}
	}

	pub unsafe fn mov_attached_down(&mut self) {
		let mut current = self.head;
		while ((current as uint) != 0) {
			if ((*current).win.attached) {
				(*current).win.blank();
				(*current).win.y = (*current).win.y + 8;
			}
			current = (*current).next;
		}
	}

	pub unsafe fn mov_attached_left(&mut self) {
		let mut current = self.head;
		while ((current as uint) != 0) {
			if ((*current).win.attached) {
				(*current).win.blank();
				(*current).win.x = (*current).win.x - 8;
			}
			current = (*current).next;
		}
	}

	pub unsafe fn mov_attached_right(&mut self) {
		let mut current = self.head;
		while ((current as uint) != 0) {
			if ((*current).win.attached) {
				(*current).win.blank();
				(*current).win.x = (*current).win.x + 8;
			}
			current = (*current).next;
		}
	}*/

	pub unsafe fn exp_attached(&mut self, direction: u32) {
		let mut current = self.head;
		while ((current as uint) != 0) {
			if ((*current).win.expanding) {
				if (direction == 0) {
					(*current).win.blank();
					(*current).win.height = (*current).win.height - 8;
				}
				else if (direction == 1) {
					(*current).win.height = (*current).win.height + 8;
				}
				else if (direction == 2) {
					(*current).win.blank();
					(*current).win.width = (*current).win.width - 8;
				}
				else {
					(*current).win.width = (*current).win.width + 8;
				}
			}
			current = (*current).next;
		}
	}

	/*pub unsafe fn exp_attached_right(&mut self) {
		let mut current = self.head;
		while ((current as uint) != 0) {
			if ((*current).win.expanding) {
				(*current).win.width = (*current).win.width + 8;
			}
			current = (*current).next;
		}
	}

	pub unsafe fn exp_attached_left(&mut self) {
		let mut current = self.head;
		while ((current as uint) != 0) {
			if ((*current).win.expanding) {
				(*current).win.blank();
				(*current).win.width = (*current).win.width - 8;
			}
			current = (*current).next;
		}
	}

	pub unsafe fn exp_attached_up(&mut self) {
		let mut current = self.head;
		while ((current as uint) != 0) {
			if ((*current).win.expanding) {
				(*current).win.blank();
				(*current).win.height = (*current).win.height - 8;
			}
			current = (*current).next;
		}
	}

	pub unsafe fn exp_attached_down(&mut self) {
		let mut current = self.head;
		while ((current as uint) != 0) {
			if ((*current).win.expanding) {
				(*current).win.height = (*current).win.height + 8;
			}
			current = (*current).next;
		}
	}*/

	pub unsafe fn add_win_back(&mut self, w: window) -> bool {
		if ((self.get_windownode(w.id) as uint) != 0) { return false; }
		let mut winnode = windownode::new(w);
		if ((self.head as uint) == 0) {
			self.head = winnode;
			return true;
		}
		let mut current = self.head;
		while (((*current).next as uint) != 0) {
			current = (*current).next;
		}
		(*current).next = winnode;
		return true;
	}

	pub unsafe fn delete_all(&mut self) {
		while ((self.head as uint) != 0) {
			(*self.head).win.blank();
			let temp = (*self.head).next;
			heap.free(self.head as *mut u8);
			self.head = temp;
		}
	}

	pub unsafe fn remove_win(&mut self, id: uint) -> bool {
		let mut current = self.head;
		if ((current as uint) == 0) {
			return false;
		}
		if (((*current).win.id) == id) {
			(*current).win.blank();
			let temp = (*current).next;
			heap.free(current as *mut u8);
			self.head = temp;
			return true;
		};
		while !((((*current).next as uint) == 0) || (((*current).next as u32) == io::BG_COLOR)) {
			if (((*(*current).next).win.id) == id) {
				(*current).win.blank();
				let temp = (*(*current).next).next;
				heap.free((*current).next as *mut u8);
				(*current).next = temp;
				return true;
			};
			current = (*current).next;
		}
		false
	}

	pub unsafe fn bring_to_top(&mut self, id: uint) -> bool {
		let mut current = self.head;
		if ((current as uint) == 0) {
			return false;
		}
		if (((*current).win.id) == id) {
			let temp = (*current).next;
			let moving_node = current;
			self.head = temp;
			if ((self.head as uint) == 0) {
				self.head = moving_node;
				(*moving_node).next = 0 as *mut windownode;
				return true;
			}
			let mut current2 = self.head;
			while (((*current2).next as uint) != 0) {
				current2 = (*current2).next;
			}
			(*current2).next = moving_node;
			(*moving_node).next = 0 as *mut windownode;
			return true;
		};
		while !((((*current).next as uint) == 0)) {
			if (((*(*current).next).win.id) == id) {
				let temp = (*(*current).next).next;
				let moving_node = (*current).next;
				(*current).next = temp;
				//if ((self.head as uint) == 0) {
				//	self.head = moving_node;
				//	return true;
				//}
				let mut current2 = self.head;
				while (((*current2).next as uint) != 0) {
					current2 = (*current2).next;
				}
				(*current2).next = moving_node;
				(*moving_node).next = 0 as *mut windownode;
				return true;
			};
			current = (*current).next;
		}
		false
	}

	/*pub unsafe fn cursor_over_window(&mut self) -> bool {
		let mut current = self.head;
		while ((current as uint) != 0) {
			if ((*current).win.cursor_within()) {
				return true;
			}
			current = (*current).next;
		}
		false
	}*/

	pub unsafe fn get_bot(&mut self) -> window {
		let current = self.head;
		return (*current).win
	}

	pub unsafe fn get_top_win(&mut self) -> *mut windownode {
		if ((self.head as uint) == 0) {
			return 0 as *mut windownode;
		}
		let mut current = self.head;
		while (((*current).next as uint) != 0) {
			current = (*current).next;
		}
		current
	}

	pub unsafe fn top_window_under_cursor(&mut self) -> *mut windownode {
		let mut current = self.head;
		let mut last = current;
		while ((current as uint) != 0) {
			if (cursor_in((*current).win.x-7, (*current).win.y-22, (*current).win.width+14, (*current).win.height+29) && (*current).win.visible) {
				last = current;
			}
			current = (*current).next;
		}
		last
	}

	pub unsafe fn draw_all(&mut self) {
		let mut current = self.head;
		while ((current as uint) != 0) {
			(*current).win.draw();
			current = (*current).next;
		}
	}
}

struct taskbar {
	menu_open: bool
}

impl taskbar {
	pub unsafe fn new() -> taskbar {
		let this = taskbar {
			menu_open: false
		};
		this
	}

	pub unsafe fn open_menu(&mut self) {
		self.menu_open = true;
	}

	pub unsafe fn close_menu(&mut self) {
		self.menu_open = false;
	}

	pub unsafe fn clear_menu(&mut self) {
		io::fill_box(0, 388, 100, 60, io::BG_COLOR);
	}

	pub unsafe fn draw(&mut self) {
		io::fill_box(1, 450, 635, 27, 0x666666);
		io::draw_box(0, 449, 637, 29, 0x000001);
		io::draw_line(57, 450, 57, 478, 0x000001);
		let mut current = winlist.head;
		let mut i: u32 = 0;
		while ((current as uint) != 0) {
			io::draw_box(58 + 70*i, 449, 70, 29, 0x000001);
			drawcstr_at_coord((*current).win.name, 66 + 70*i, 456, 0x000001);
			current = (*current).next;
			i += 1;
		}
		if (self.menu_open) {
			io::fill_box(0, 428, 100, 20, 0x888888);
			io::draw_box(0, 428, 100, 20, 0x000000);
			drawstr_at_coord(&"Open Window", 6, 430, 0x000000);
			io::fill_box(0, 408, 100, 20, 0x888888);
			io::draw_box(0, 408, 100, 20, 0x000000);
			drawstr_at_coord(&"Close All", 14, 410, 0x000000);
			io::fill_box(0, 388, 100, 20, 0x888888);
			io::draw_box(0, 388, 100, 20, 0x000000);
			drawstr_at_coord(&"Show Files", 10, 390, 0x000000);
		}
		drawstr_at_coord(&"Menu", 11, 456, 0x000001);
	}
}
