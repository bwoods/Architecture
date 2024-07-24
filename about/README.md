	alias ~~~=":<<'~~~sh'";:<<'~~~sh'

# Publishing

<small>Here are the steps required to create (and publish) a staging copy of the documentation that will eventually be published to the [docs.rs]() page for this crate.</small>[^sh]




[^sh]: Because this README file [is also a valid Bourne shell script](https://gist.github.com/bwoods/1c25cb7723a06a076c2152a2781d4d49), sourcing it will do these steps automatically.



## Ensure Semantic Versioning

As this crate has not been published yet, for now, we must explicitly set the baseline to compare the public interface to.

~~~sh
cargo semver-checks --baseline-rev v0.6.0 || exit
~~~

If there are any [detected violations](https://github.com/obi1kenobi/cargo-semver-checks) of SemVar, the crate should not be published until the version number has been updated accordingly. Or if the change was accidental, the breakage should be fixed.

> [!TIP]
>
> - If the version number does need to be updated, remember to change it in the [top-level README](../README.md) as well.
> - `cargo semver-checks` itself can be updated with `cargo install-update -a`.
>   - Assuming `cargo install cargo-update` was installed.



## Generating the Documentation

A fresh build of all of the crates documentation is performed; ensuring that no “out of date” files are left in place.	

~~~sh
# These steps should be done within the 'about' directory
cd "$(git rev-parse --show-toplevel)/about" || exit
echo ".lock" > .git/info/exclude # ignore the lock file
~~~

The nightly version of **rustdoc** is used so that the unstable `feature(doc_auto_cfg)` can be used to [indicate feature-gated items in documentation](https://github.com/rust-random/rand/issues/986). Look for the `docsrs` flag in the crate source to see how it is used.

~~~sh
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --no-deps --all-features || exit
~~~

Github Pages requires an `index.html` page at the root of the documentation branch; whereas **rustdoc** nests it within a folder named after the crate. [As discussed here](https://dev.to/deciduously/prepare-your-rust-api-docs-for-github-pages-2n5i), a simple redirect can be put in place.

~~~sh
echo "<meta http-equiv='refresh' content='0; url=composable'>" \
   > ../target/doc/index.html
~~~



### Pushing to the documentation branch

Now that the documentation has been generated it must be push to the [appropriate branch](https://github.com/bwoods/Architecture/tree/docs.rs) on GitHub.

~~~sh
cd ../target/doc/
git init --quiet --initial-branch=docs.rs
rm .lock # remove the lock file; we won't need it
~~~

Although git is being used to manage the documentation files there is no needs to preserve the history of this branch. It is recreated every time.

~~~sh
git add --all
git commit --quiet --allow-empty-message -m ""
~~~

After all of the file have been added, they are pushed to the remote branch.

~~~sh
git remote add -m docs.rs github https://github.com/bwoods/Architecture.git
git push --force --set-upstream github docs.rs
~~~

Since this branch share no history with any previous version pushed to the repository, a `--force` push is required.



### Viewing the documentation

Eventually, the crate documentation should be visible at the [GitHub Pages url for the repository](http://bwoods.github.io/Architecture).

- The appropriate branch and root will need to be selected in the repositories [Pages settings](https://github.com/bwoods/Architecture/settings/pages).



> [!NOTE]
>
> It can take up to 10 minutes for changes to your site to publish after you push the changes to GitHub.&nbsp; [↩](https://docs.github.com/en/pages/getting-started-with-github-pages/creating-a-github-pages-site)



> [!WARNING]
>
> GitHub Pages sites are publicly available on the internet, even if the repository for the site is private. If you have sensitive data in your site's repository, you may want to remove the data before publishing. For more information, see “[About repositories](https://docs.github.com/en/repositories/creating-and-managing-repositories/about-repositories#about-repository-visibility).”&nbsp; [↩](https://docs.github.com/en/pages/getting-started-with-github-pages/creating-a-github-pages-site)



