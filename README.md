# lbl

LBL (line-by-line) is a very simple line-based and console-based text editor.  
See below for an exemplary workflow.

## Building

**Requirements:** [Rust & Cargo](https://www.rust-lang.org/).

Run `cargo build --release` to compile the project.
Find the executable under `./target/release/lbl(.exe)`.

## Usage

LBL has just a single (optional) argument: the file you wish to edit.  
When you don't give it a file you will start working on an empty, unnamed file instead.
```
lbl [file]
```

If you later wish to edit an existing file you don't need to restart LBL completely. Instead
you can use the `O` (Open) command. The `O` command takes a single argument: the file name.
```
! O poem.txt
```

Now you can view the file using the `L` (List) command.
```
! L
<empty>
```
You can also give it a range of lines to display:
```
! O error.log
! L 10-12
   10| Error in file main.c:
   11|     Invalid parameters to function `add'
   12| in function `main'
```

To add new content to a file you use the `A` (Append) command. This command will allow you to enter new text
line by line.  
To stop editing, send the EOF signal (Ctrl+D on Unix-like, Ctrl+Z on Windows).
```
! A
   0| Roses are red
   1| Violets are blue
   2| I'm writing a poem
   3| What about you?
   4| ^D
! 
```

Now to save the file you use the `S` (Save) command. The `S` command takes an optional or required argument,
which will be used as the file path. If you are editing an unnamed file (i.e. you didn't previously use the `O` command)
you need to supply a file name. If you are editing a named file you can leave it empty and it will update the open file.
```
! S
! S poem2.txt
```

If you've made a mistake you'd like to correct there's also the `E` (Edit) command.  
Give it a line number and you will begin inserting lines at this line number.  
This does not allow you to jump ahead in the file though. If the line number is bigger than the line count of the file
it will simply put you right behind the last line in the file.
```
! L
   0| Roses are red
   1| Violets are blue
   2| I'm writing a poem
   3| What about you?
! E 1
   1| Violets are violet
   2| ^D
! L
   0| Roses are red
   1| Violets are violet
   2| I'm writing a poem
   3| What about you?
```

To empty a file you can use the `C` (Clear) command.
```
! L
   0| Roses are red
   1| Violets are violet
   2| I'm writing a poem
   3| What about you?
! C
! L
<empty>
```

When you're done editing you can quit using the `Q` (Quit) command.

You can display an overview of the available commands using the `H` (Help) command.

## License

Licensed under the [MIT License](LICENSE.md).
