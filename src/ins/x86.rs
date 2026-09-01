ins_mod!(rdpid as pub);
ins_mod!(rdtscp as pub);
ins_mod!(rdtsc as pub);
ins_mod!(cpuid as pub);
ins_mod!(msr as pub);
ins_mod!(hlt as pub);
ins_mod!(flags as pub);
ins_mod!(fence as pub);
ins_mod!(tab as pub);
ins_mod!(random as pub);
ins_mod!(tr as pub);
ins_mod!(int); // `int`'s `#[macro_export]` makes `int!` available at this module.
ins_mod!(pub port);
