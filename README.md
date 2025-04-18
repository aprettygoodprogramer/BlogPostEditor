# What is This?

A simple blog post editor for my blog!

# How can I run it

Idk why you would ever want to run it, but if you do you can do it like this:


Make sure you have git installed, and then run

```bash
git clone https://github.com/aprettygoodprogramer/BlogPostEditor.git
cd BlogPostEditor
```

Once you've done this you need to create a .env file:


Windows
```bash
echo.>.env
```

Linux/MacOs

```bash
touch .env
```

Then, with your editor of choice, go into it and assign two values:

```bash
DATABASE_URL=yourDataBaseUrl
PATH_SAVE=the/path/you/want/to/save/to
```

Once you've done that do,

```bash
cargo run
```

And you're done!!!

If someone is reading this, feel free to make issues or feature requests!!!
