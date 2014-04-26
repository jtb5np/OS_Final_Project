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

pub static mut arr1: bool = false;

pub static mut arr2: bool = false;


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

pub unsafe fn drawstr(msg: &str) {
    let old_fg = super::super::io::FG_COLOR;
    let mut x: u32 = 0x6699AAFF;
    for c in slice::iter(as_bytes(msg)) {
	x = (x << 8) + (x >> 24); 
	super::super::io::set_fg(x);
	drawchar(*c as char);
    }
    super::super::io::set_fg(old_fg);
}

pub unsafe fn drawcstr(s: cstr)
{
    let mut p = s.p as uint;
    while *(p as *char) != '\0'
    {	
	drawchar(*(p as *char));
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

pub unsafe fn putcstr(s: cstr)
{
    let mut p = s.p as uint;
    while *(p as *char) != '\0'
    {	
	putchar(*(p as *char));
	p += 1;
    }
}

pub unsafe fn parsekey(x: char) {
	let x = x as u8;
	// Set this to false to learn the keycodes of various keys!
	// Key codes are printed backwards because life is hard	
	if (true) {
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
				bring_window_to_top(win.name);
				io::draw_cursor();
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
	    				if (winlist.cursor_over_window())
	    				{
						winlist.draw_all();
	    				}
					io::move_cursor_up();
	    				io::draw_cursor();
					arr2 = false;
				}
			}
			0x42	=>	{
				if (arr2) {
					io::blank_cursor();
	    				if (winlist.cursor_over_window())
	    				{
						winlist.draw_all();
	    				}
	   				io::move_cursor_down();
	    				io::draw_cursor();
					arr2 = false;
				}
			}
			0x43	=>	{
				if (arr2) {
					io::blank_cursor();
	    				if (winlist.cursor_over_window())
	    				{
						winlist.draw_all();
	    				}
	   				io::move_cursor_right();
	    				io::draw_cursor();
					arr2 = false;
				}
			}
			0x44	=>	{
				if (arr2) {
					io::blank_cursor();
	    				if (winlist.cursor_over_window())
	    				{
						winlist.draw_all();
	    				}
	   				io::move_cursor_left();
	    				io::draw_cursor();
					arr2 = false;
				}
			}
			_		=>	{
				if (buffer.add_char(x)) { 
					putchar(x as char);
					//drawchar(x as char);
				}
			}
		}
	}
	else {
		keycode(x);
	}
}

unsafe fn drawchar(x: char)
{
	if x == '\n' {
		io::CURSOR_Y += io::CURSOR_HEIGHT;
		io::CURSOR_X = 0u32;
		return;
	}
    
    io::restore();
    io::draw_char(x);
    io::CURSOR_X += io::CURSOR_WIDTH;
    if io::CURSOR_X >= io::SCREEN_WIDTH {io::CURSOR_X -= io::SCREEN_WIDTH; io::CURSOR_Y += io::CURSOR_HEIGHT}
    io::backup();
    io::draw_cursor();
}

unsafe fn drawchar_at_coord(c: char, x: u32, y: u32, color: u32)
{    
    io::draw_char_at(c, x, y, color);
}

fn keycode(x: u8) {
	//let mut x = x;
	//while  x != 0 {
	//	putchar((x%10+ ('0' as u8) ) as char);
	//	x = x/10;
	//}
	//putchar(' ');
}

pub unsafe fn init() {
    buffer = cstr::new(256);
    let root_name = cstr::from_str(&"root");
    let root_dir = directory::new(root_name, 0 as *mut dirnode);
    let winlist = windowlist::new();
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
	//putstr("\n");
	//putcstr(buffer);
	//drawstr("\n");
	//drawcstr(buffer);
	//blah
	if (buffer.streq(&"ls")) {
	    list_directory(wd);
	}
	else if (buffer.streq(&"pwd")) { 
	    let mut pwd = cstr::new(4096);
	    let mut slash = cstr::from_str(&"/");
	    let mut current = wd;
	    while !(((current as uint) == 0) || ((current as u32) == io::BG_COLOR)) {
		pwd.concatfront((*current).di.dname);
		pwd.concatfront(slash);
		current = (*current).di.parent;
	    }
	    putstr("\n");
	    //drawstr("\n");
	    putcstr(pwd);
	    //drawcstr(pwd);
	}
	else if (buffer.streq(&"click")) {
	    let mut w = window::new(cstr::from_str("Window 1"), 200, 50, 200, 200, true);
	    let mut w2 = window::new(cstr::from_str("Window 2"), 300, 200, 200, 150, true);
	    winlist.add_win_front(w);
	    winlist.add_win_front(w2);
	    bring_window_to_top(w.name);
	    //remove_window(w.name);
	    putstr("\n");
	    putstr("clicked");
	}
	else if (buffer.streq(&"movewin")) {
		move_window((*winlist.head).win.name, (*winlist.head).win.x + 10, (*winlist.head).win.y + 10);
	}
	else {
		match buffer.getarg(' ', 0) {
		    Some(y)        => {
			if(y.streq(&"cat")) {
			    match buffer.getarg(' ', 1) {
				Some(x)        => {
				    let catout = read_file(x);
				    if !(catout.streq(&"")) {
					putstr("\n");
				  	//drawstr("\n");
				    	putcstr(catout);
				    	//drawcstr(catout);
				    }
				}
				None        => { }
			    };
			}
			else if(y.streq(&"mv")) {
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
			else if(y.streq(&"echo")) {
				let (a,b) = buffer.splitonce(' ');
				putstr("\n");
				putcstr(b); 
				//drawstr("\n");
				//drawcstr(b);
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
				else {
					putstr("\nThat directory doesn't exist.");
					//drawstr("\nThat directory doesn't exist.");
				}
			    }
			}
			else if(y.streq(&"rm")) {
			    match buffer.getarg(' ', 1) {
				Some(x)        => {
				    if !(delete_file(wd, x) || delete_directory((*wd).di.get_child(x))) {
					putstr("\nNo such file/directory or directory isn't empty.");
					//drawstr("\nNo such file/directory or directory isn't empty.");
				    }
				}
				None        => { }
			    };
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
							write_file(filename, words);
						}
						else {
							create_file(wd, filename);
							write_file(filename, words);
						}
					}
					None        => { }
				    };
				}
				None        => { }
			    };
			}
			else {
			    putstr(&"\nNO");
			    //drawstr(&"\nNO");
			}
		    }
		    None        => { }
		};
	};
	winlist.draw_all();
	io::draw_cursor();
	buffer.reset();
}

unsafe fn read_file(filename: cstr) -> cstr {
	(*wd).di.read_file(filename)
}

unsafe fn write_file(filename: cstr, word: cstr) -> bool {
	(*wd).di.write_file(filename, word)
}

unsafe fn create_file(dir: *mut dirnode, name: cstr) {
	let mut newfile = file::new(1024, name);
	(*dir).di.add_file(newfile);
}

unsafe fn delete_file(dir: *mut dirnode, name: cstr) -> bool {
	(*dir).di.remove_file(name)
}

unsafe fn list_directory(dir: *mut dirnode) {
	(*dir).di.directory_file_list()
}

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
			//drawstr("\n");
			//drawcstr((*current).di.dname);
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
			//drawstr("\n");
			//drawcstr((*(*current).next).fi.fname);
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
				putstr(&"\nSomething broke!");
				return None; 
			}
		}
	}

#[allow(dead_code)]
	unsafe fn split(&self, delim: char) -> (cstr, cstr) {
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
	}

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


pub unsafe fn move_window(wname: cstr, x: u32, y: u32) {
	let wndw = winlist.get_windownode(wname);
	(*wndw).win.blank();
	(*wndw).win.mov(x, y);
	winlist.draw_all();
}

pub unsafe fn bring_window_to_top(wname: cstr) {
	winlist.bring_to_top(wname);
	winlist.draw_all();
}

struct window {
	name: cstr,
	x: u32,
	y: u32,
	width: u32,
	height: u32,
	visible: bool
}

impl window {
	pub unsafe fn new(name: cstr, x: u32, y: u32, width: u32, height: u32, visible: bool) -> window {
		let this = window {
			name: name,
			x: x,
			y: y,
			width: width,
			height: height,
			visible: visible
		};
		this
	}

	pub unsafe fn set_visible(&mut self, vi: bool) {
		self.visible = vi;
	}

	pub unsafe fn mov(&mut self, x: u32, y: u32) {
		self.x = x;
		self.y = y;
	}

	pub unsafe fn cursor_within(&mut self) -> bool {
		(io::CURSOR_X > (self.x-7-io::CURSOR_WIDTH)) && (io::CURSOR_X < (self.x+self.width+12)) && (io::CURSOR_Y > (self.y-22- io::CURSOR_HEIGHT)) && (io::CURSOR_Y < (self.y+self.height+7))
	}

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

	pub unsafe fn get_windownode(&mut self, name: cstr) -> *mut windownode {
		let mut current = self.head;
		while ((current as uint) != 0) {
			if ((*current).win.name.eq(&name)) {
				return current;
			}
			current = (*current).next;
		}
		current
	}

	pub unsafe fn add_win_front(&mut self, w: window) -> bool {
		if ((self.get_windownode(w.name) as uint) != 0) { return false; }
		let mut winnode = windownode::new(w);
		if ((self.head as uint) == 0) {
			self.head = winnode;
		}
		else {
			let temp = self.head;
			(*winnode).next = temp;
			self.head = winnode;
		}
		true
	}

	pub unsafe fn add_win_back(&mut self, w: window) -> bool {
		if ((self.get_windownode(w.name) as uint) != 0) { return false; }
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

	pub unsafe fn remove_win(&mut self, name: cstr) -> bool {
		let mut current = self.head;
		if ((current as uint) == 0) {
			return false;
		}
		if (((*current).win.name).eq(&name)) {
			(*current).win.blank();
			let temp = (*current).next;
			heap.free(current as *mut u8);
			self.head = temp;
			return true;
		};
		while !((((*current).next as uint) == 0) || (((*current).next as u32) == io::BG_COLOR)) {
			if (((*(*current).next).win.name).eq(&name)) {
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

	pub unsafe fn bring_to_top(&mut self, name: cstr) -> bool {
		let mut current = self.head;
		if ((current as uint) == 0) {
			return false;
		}
		if (((*current).win.name).eq(&name)) {
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
			if (((*(*current).next).win.name).eq(&name)) {
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

	pub unsafe fn cursor_over_window(&mut self) -> bool {
		let mut current = self.head;
		while ((current as uint) != 0) {
			if ((*current).win.cursor_within()) {
				return true;
			}
			current = (*current).next;
		}
		false
	}

	pub unsafe fn get_bot(&mut self) -> window {
		let current = self.head;
		return (*current).win
	}

	pub unsafe fn draw_all(&mut self) {
		let mut current = self.head;
		while ((current as uint) != 0) {
			(*current).win.draw();
			current = (*current).next;
		}
	}
}
