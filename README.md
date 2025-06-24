# Rewind
A simple reminder app in order to learn GUIs, Storing Data, and the concepts thereof

# THIS IS A WORK IN PROGRESS, and may never be finished.
This is only (Mainly) built for the GNOME desktop environment on Linux , so issues on other DEs/OSes will likely happen, and if possible, using Linux(which GTK is built for) will lead to an overall better experience.


## Installation
##### Linux:
   Install Rustup/Rust from your package manager or from the website, along with GTK4, GTK4-devel, and gcc, these may be already installed if you've done development on your system. installation will look similar to this:(ex: `sudo dnf install rustup gtk-devel gcc`, or `sudo pacman -S gtk4-devel gcc`) 

   Run the following Terminal in your shell ``git clone https://github.com/thesillyboi/Rewind && cd Rewind && cargo build --release && mv target/release/Rewind ~/.cargo/bin/ | sh`` You can also put the binary in another location in your PATH instead of .Cargo/bin, but that's where I'd reccomend storing it.
   Finally, run `Rewind` in your shell to use the program.

##### Windows/Mac:
   Good luck, If you can get it working on Windows or Mac, please make a pull request with build instructions, as I do not have a computer running either of them for testing, so Windows and Mac support is very theoretical.


If there are missing Icons within the header bar or other areas of the app, it's because I didn't bundle the icons into the app, and instead used ones built into GNOME, if you know how to bundle them in, please help me with doing so.

*This work was assisted using the use of Artificial Intelligence Models, specifically Anthropic's Claude Sonnet 4.0 model*


-Adrian, thanks for checking this out