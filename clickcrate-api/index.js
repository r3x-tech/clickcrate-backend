const PORT = process.env.PORT || 3000;
const HOST = process.env.HOST || "http://localhost:3000";

app.listen(PORT, () => {
  console.log(`ClickCrate API server listening on ${HOST}`);
});
