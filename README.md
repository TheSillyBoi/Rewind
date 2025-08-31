[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT][license-shield]][license-url]
[![AI Assisted Yes!](https://img.shields.io/badge/AI%20Assisted-yes!-red?style=for-the-badge)](https://github.com/mefengl/made-by-ai)


# Rewind
A simple reminder app in order to learn GUIs, Storing Data, and the concepts thereof

## Disclaimers:
1. This is only (Mainly) built for the GNOME desktop environment on Linux , so issues on other DEs and OSes will likely happen, and if possible, using GNOME and Linux will lead to an overall better experience.

2. This mainly just a passion project, and may go unmaintained for long periods of time(I'll try to keep working on it tho) 

3. There have been issues in the past of this not giving notifications, or having them not pop up

4. There is no background service for Rewind yet, it will need to be left open to recieve reminders

## **Installation Instructions**
### Linux:

> Rewind is not in any Package Repos
   #### compiling from source: 
   Install Rustup/Rust from your package manager or preferably from the website, along with GTK4, GTK4-devel, and gcc(or another C compiler of choice), these may be already installed if you've done development on your system. installation will look similar to this:(ex: `sudo dnf install rustup gtk-devel gcc`, or `sudo pacman -S gtk4-devel gcc`) 

   Run the following command in your shell ``git clone https://github.com/thesillyboi/Rewind && cd Rewind && cargo build --release| sh`` You can also put the binary  in your PATH, finally, run `./Rewind` in your shell to use the program(if you don't watch the ./ you'll need it in your PATH).

   ### Precompiled Binaries
   I will add releases with a Linux Binary, but they may fall out of date

   #### Windows/Mac:
   Rewind has not been tested for Windows or OS X:(


## Usage
   ### Adding a Reminder:
   1. Press the + in the topbar of the app
   2. In the first textbox, write what you want your reminder to be called(ex: Walk the dog, Water Plants, Study for Math Exam, etc)
   3. Click on the second textbox(only supports integers 0-23), this represents what hour you want your reminder to be given(It's 24 hour clock only atm)
   4. Click the third textbox(only supports integers 0-59), this represents at what minute you want your reminder to be given.
   5. Select the date on the calendar, press the arrows to switch months and years
   6. Reread it, before pressing the checkmark, which directly adds it to the main UI
   ### Deleting a reminder:
   1. Press "Delete Reminder" under the reminder you want to get rid of
   2. That's All :p
   ### Viewing Credits:
   1. Press the Menu button in the topbar of the app(next to the +)
   2. Press About
   3. You can navigate by clickign on "About, Credits, or License"
   ### Exiting the App:
   1. Press the X button in the top right



--- 
## Thanks for the support of this! ❤️ -Adrian Tennies
[contributors-shield]: https://img.shields.io/github/contributors/thesillyboi/rewind.svg?style=for-the-badge
[contributors-url]: https://github.com/thesillyboi/rewind/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/thesillyboi/rewind.svg?style=for-the-badge
[forks-url]: https://github.com/thesillyboi/rewind/network/members
[stars-shield]: https://img.shields.io/github/stars/thesillyboi/rewind.svg?style=for-the-badge
[stars-url]: https://github.com/thesillyboi/rewind/stargazers
[issues-shield]: https://img.shields.io/github/issues/thesillyboi/rewind.svg?style=for-the-badge
[issues-url]: https://github.com/thesillyboi/rewind/issues
[license-shield]: https://img.shields.io/github/license/thesillyboi/rewind.svg?style=for-the-badge
[license-url]: https://github.com/thesillyboi/rewind/blob/master/LICENSE.txt