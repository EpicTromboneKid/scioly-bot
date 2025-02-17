# SciolyBot

## This project is currently in active development. Refer to the sections below for a more detailed list of features.

SciolyBot is a discord bot for **automatic + unattended test administration**, with a focus on providing a hassle-free test-taking experience.
It is currently optimized for a workflow in Science Olympiad, but can be _easily adapted to other use cases_.

### Here are some currently supported features...
- **auto-role detection** in servers to provide _personalized_ tests ✏️
- setting default emails for hassle-free use with google docs, where the answers are saved
- **partner detection** and sharing when starting a test
- automatic sharing with administrators 🔗
- test administration configured by the administrator
- automatic permission changing after time expires, with a reminder sent in channel at 5 minutes remaining
- auto-test submission,
- progress checks,
- and more!

### ...and some features in the works!
- [ ] time tracking (timestamps of start and finish, etc.),
- [ ] automatic team and event assignments through team rosters,
- [ ] and an ai chatbot to aid in grading and giving test feedback!

(This is by no means an exhaustive list, and it will be updated as I plan more changes.)

### Final thoughts!
>[!NOTE]
>If you want to try out this bot, send me a dm on discord @epictrombonekid. I haven't put the official add link here, because it's still not completely implemented and bug-free.

>[!CAUTION]
>If you're here from crates.io, be sure to set the BOT_TOKEN and SERVICE_ACCOUNT_CREDS account variables. The BOT_TOKEN should be a string, and the SERVICE_ACCOUNT_CREDS should be the path to a service account json file.

If you'd like to propose any additional features/improvements/bug fixes, don't hesitate to make a pull request or open an issue!
I'm open to any and all feedback :)

_This project is built entirely in Rust._
