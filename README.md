# maidbot

A multi-purpose, all-in-one discord bot \
⚠️ (code is very :sob:)

### Key Features

- Action commands, such as /action kiss, hug, etc.
- User info commands (avatar, banner, etc)

# Configuring the bot

- Install all dependencies beforehand:

```bash
npm install
```

- Create a .env file in the project folder. Example of .env file:

```env
CLIENT_TOKEN=ABCDEFGH
```

- Modify `constants.ts` file as you wish.

# Deploying the bot

On dev environment:

```bash
npm run dev
```

On production (immediately runs from dist, assuming everything has been compiled beforehand)

```bash
npm run start
```
