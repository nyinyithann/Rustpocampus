# Notes

> Rust programs don't usually explicitly drop values at all, in the way C and C++ programs would use free and delete. The way to drop a value in Rust is to remove it from the ownership tree somehow: by leaving the scope of a variable, or deleting an element from a vector, or something of that sort. At that point, Rust ensures the value is properly dropped, along with everything it owns.

> Weak references, which are references that do not increase the reference count of an object, allow objects to have circular references without causing memory leaks.
