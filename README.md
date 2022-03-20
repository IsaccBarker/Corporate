# Corperate, the esoteric programming language for large businesses.

<p align="center">
    <img src="https://i.giphy.com/l41YfsMeWbVPC5XPy.gif">
</p>

Are you tired of having to write **serious** corperate emails? Me neither! With the corperate language,
your pointlessly formal and not at all understandable communications can also function as full fledged
language. Take that HR!

Corperate is a Object Oriented (because it has the word "enterprise" in it, not because it's actually
the right tool), with some stack based programming language capabilities.

## Building
Build it like you would any Rust program; install the right tool chain, build, and run! If you don't have
Rust already installed, you can visit [rustup.rs](https://rustup.rs/). You don't even need to clone this
repository to build Corperate, simply run the command below to fetch, build, and install:

```bash
cargo install --git https://github.com/IsaccBarker/Corperate.git
```

It's that easy!

## How it works

Corperate is a turing complete language, which means you can calculate anything you could want with it.
verything your heart can desire! It can't yet actually interact with the system itself (besides writting
to stdout), but we're working on that ;)

Corperate scripts should end in the `.email`, `.msg`, or `.eml` file extension. However, corperate scripts
cannot call other corperate scripts.

### Sound like a human

Programming languages typically do not sound like human speach, although some try (see AppleScript), most fail.
Corperate actually makes you sound like a real person. See the below email exchange between Vivian and Ainsley.

```
Subject: Project Delay (Payment Cluster Deployment)
Original Recipient: N/A
Date: Jule 28, 2028, 11:03 AM

Hi Ainsley,
    Regretfully, I must bring to attension two bugs that might effect our bottom line, as fixing these would require
a substantial delay. Following up on your meeting, would you be willing to commucate in person once more? Just wanted
to run changing the project deadline up the flagpoll, and I hope we can sort this out and not involve upper management,
but if nessisary I'll ping Claire and Carolina and have them in the loop regarding this chain. Lets consider our clients
first and foremost, but we may have have to table this to reach our bottom line. I'd like to sincerely apologize to our
clients, if this doesn't work out.

    Regretfully,
    Vivian



Subject: Project Delay (Payment Cluster Deployment)
Original Recipient: Ainsley
Date: July 28, 2028, 10:28 AM

Hey Vivian,
    Hope you're having a wonderful day, and that this email finds you well. Concerning those two points you've brought up
concerning an unexpected that you and your team have encounted. We don't currently have any leway here, as our next deployment
is only four days away. With two more serious production-exclusive bugs, I'm afraid that this won't track well with your
record here. We can keep deploying the legacy code, but that would require us providing very slow service. It's always an option,
but it would be much better if you could have your team crunch, and get us the software we need to keep providing the exelent
service that this company is known for. We may have to incude upper management if this doesn't work out.

    Kind regards,
    Ainsley
```

### Start

Code execution will always start with the first class, and the first method inside that class.

### Eggs

`<eggs>` is used to signifiy that anything at all can go here (sans a new line). It will simply look for the word it
expects next, and continue from there. That way, it's possible to have full creative expression in your locked down
corperate email.

### Classes

Corperate is a fully Object Oriented language. This is not because the developers want it, or because it's the best
choice for the job, but instead because Java has the word "Enterprise" next to it. Everything is a class, except for
the primitive Integer and String.

A class is simply a new email. Take the above chain as an example; we have both the classes "Hey" and "Hi". You can
use these classes to house methods, and thus functionality. A new class is marked with three new lines, `Subject`
with no tab before it, followed by `Original Recipient` and `Date`. After that, is the start of a new class/email.

The name of a class is taken either from the parentheses after the subject, or from the date ("yesterday", "2 days ago",
"1 week and 2 days ago") are all perfectly valid.

All class variables are public, and you can assign to them like so:

```
remember
<probably not> forget
```

You cannot pre-define class variables in Corperate. You just have to run a method, and hope that you convey your message.
That's part of the fun!

### Methods

Each method is defined by a block of text starting with a tab and ending with a new line. The name of the method is
the number of words (delimiters " ", ",", ":", ";", "--", "(",  and ")") that are present in the first sentense. Static
methods do not exist.

You can call a method in another class by saying:

```
(<eggs>see the <method name><eggs> interation stated <class ID><eggs>)
, namely the <method name><eggs> requirement <eggs> referenced <class ID>
. Following up on your <class ID>, would you be willing to <method name>?
```

The `--` (double hyphen) might look weird, but it's [perfectly valid](https://english.stackexchange.com/a/254370). You can
also specify a class one email in the future with `<x>`. You can reference classes up to 10 emails in the future,
by the following:

1. x = 1, "meeting"
2. x = 2, "conference"
3. x = 3, "chat"
4. x = 4, "call"
5. x = 5, "note"
6. x = 6, "letter"
7. x = 7, "download"
8. x = 8, "uplink"
9. x = 9, "conversation"
10. x = 10, "consultation"

You can call a method in the same class by saying:

```
(<eggs>see the <method name><eggs> iteration stated in this email<eggs>)

or

, namely the <method name><eggs> requirement <eggs> refered in this email
```

## Types

There are only three types and constants in Corperate, those being the Integer, the String, and the Class. String are
enclosed within quotes, and integers being just numbers. Classes are different.

To define a class, you must name the class (see the class section of this document), then say:

```
on the email <eggs> <class ID>
loop back into <eggs> <class ID>
```

When referencing a class, please put the word `reference` before the class identifier.

If you don't wish to use a Integer literal, as it might not sound professional, you can do what you might do in the
Rockstar programming language and modulo 10 the length of words, followed up by a period:

```
don't feel comfortable

4 % 10   4 % 10   10 % 10

440
```

## Variables

You can define a variable like so:

```
put <variable name> on the docket
```

You can set the value of a variable to be:

```
request that <variable name> be <constant>
```

## Number of elements

The number of elements (commonly used in relation to the stack), is a comma seperated list of whatever,
ending with an [ofxord comma](https://www.grammarly.com/blog/what-is-the-oxford-comma-and-why-do-people-care-so-much-about-it/)
and an and (`, and`).

## Things

You may see `<things>`, and this is a reference to a list of variables or constants. It follows the
same convension as number of elements.

## The stack

You can push things onto the stack with:

```
I'd like to consider <things>
bring to attension <things>
```

You can also pop things from the stack like so:

```
lets drop <number of element>
I don't have the bandwidth to <eggs> <thing>
```

To store a reference to a variable so that the top element in the stack can be stored inside it, you can:

```
let's consider <things>
```

You can store the top of the stack as a variable with:

```
table this
put a pin in it
loop back to this
```

## Arithmetic

Because Corperate is a stack based language, arthithmetic if pretty simple.

The number of elements refers to how many items on the stack do we want to 

```
addition:
add <number of elements>
include <number of elements>
. With <number of elements>

subtraction:
subtract <number of elements>
discontinue <number of elements>
ignore <number of elements>
. Without <number of elements>

multiplication:
multiple <number of elements>
times <number of elements>
emphasize <number of elements>
boost <number of elements>
. Of <number of elements>

division:
divide <number of elements>
controversial between <number of elements>
don't like <number of elements>
. Over <number of elements>
```

The result will be popped onto the stack, and the elements that are used
will be popped off. You can then store this in a variable.

## Printing

You can print to the console by specifying a key word, and then the variable
you want to print.

```
apologize to <variable>
state <variable>
tell <variable>
say <variable>
print <variable>
```

# That's about it!

