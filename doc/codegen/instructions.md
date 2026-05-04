### Arithmetic operations:

These commands take two items (a / b / <>) from stack and put result on top of stack (e. g. a - b)
add - returns int if both arguments are ints. Otherwise, real
sub - returns int if both arguments are ints. Otherwise, real
mul - returns int if both arguments are ints. Otherwise, real
div - returns int if both arguments are ints. Otherwise, real
mod - runtime checks for both arguments to be ints

### Boolean operations:
Only take real, int, bool, list (only for eq and neq) as arguments. Puts bool on top of stack
eq
neq
less
leq
greater
geq

### Jumps:

jump <label>
condjump <label> - takes bool from top of stack and jumps on true
<label>: - label to jump to

### Functions:
func <funclabel> {
<captures> - interpreter binds addr of captures to func
	<args>
	<locals>

	<body>

  }
capture: capture <varlabel>, … (captures addr of label)
args: arg <varlabel>, …
locals: local <varlabel>, …
Body: if no return, return null
loadlocal <varlabel> - load value of variable to top of stack
loadcapture <varlabel>
loadarg <varlabel>
loadglobal <varlabel>
setlocal <varlable> - sets value to local variable value of top of stack
setcapture <varlabel> - 
setarg <varlabel>
setglobal <varlabel>
call <funclabel> - calls <funclabel> with args on top of stack (checks number of args to call)
callstack - calls function from top of stack with args
ret - returns single elem from stack (if more than one -> runtime error) 

### Globals:
	global <varlabel>

### Consts:
Loads values/addrs to the top of stack
loadnull
loadint <value>
loadreal <value>
loadbool <value>
loadfunc <funclabel> - finds <funclabel> addr and puts it on top of stack (for callstack)
makelist <n> - takes n args from top of stack and puts single list with <n> elems

### StdLib
Separate file with bytecode for stdlib. Metadata for the file should contain all labels and (preloaded or loaded on demand?) loaded for execution.

### Virtual Function
createvfunc <newfunclabel> <basefunclabel>
Its own <newfunclabel>
Label on base function
Captured Args
setcapturearg <funclabel> <varlabel> - binds addr of var`label to the first arg
setcaptureargconst <real,int,bool,list> - value from stack
