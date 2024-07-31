module.exports = {
  apps: [
    {
      name: "http-server",
      script: `SECRET_ENC=${process.env.SECRET_ENC} node dist/entrypoints/server.js`
    }
  ]
}
