flood: flood.rs
	rustc -O $<
	strip $@
