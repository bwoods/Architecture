
	alias ~~~=":<<'~~~sh'";:<<'~~~sh'

<img src="https://imgs.xkcd.com/comics/e_to_the_pi_minus_pi.png" style="zoom:150%;" />

<small>Comparing the text of the SVG files implicitly assumes that Rust will round the floating point values the same across various architectures and compiler releases. We’ll deal with that assumption when it fails…</small>



# Viewing `Output` testing output

This folder contains the output of running the [insta](https://docs.rs/insta/latest/insta/) snapshot tests that are written for the `views::Output` trait. Running the snapshot tests, either directly

```sh
$ cargo insta test --review --all-features
```

or as part of the unit tests, will produce snapshots and compare them to the ones saved here. However, it can be convenient to actually look at the snapshots to confirm that they match expectations. 

Given that the format of each **.snap** file is a YAML head followed by the content of the snapshot, and our snapshots are valid SVG files, out snapshots are valid Markdown files and can be viewed in any graphical Markdown viewer.

```sh
$ open -a Typora *.snap
```



## Producing SVG files

Granted, a more straightforward option would be to convert the **.snap** files into actual SVG files so that they may be viewed in any application that can view SVGs.[^sh]

This expression will convert all the “expectation” files into SVGs

~~~sh
for snap in *.snap; do
    tail +5 "$snap"  > $snap.svg
done
~~~

and this (slight variation)

~~~sh
for snap in *.snap.new; do
	[ -e "$snap" ] || continue # ignore missing files
    tail +6 "$snap"  > "$snap.svg"
done
~~~

will convert all of the files produced by failed tests. Both sets of SVGs could then be visually inspected. 



[^sh]: Because this README file [is also a valid shell script](https://gist.github.com/bwoods/1c25cb7723a06a076c2152a2781d4d49), running it will do exactly that.
