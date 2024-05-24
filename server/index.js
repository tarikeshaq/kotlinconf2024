const express = require('express');
const bodyParser = require('body-parser');
const cors = require('cors');
const app = express();
app.use(cors());
app.use(bodyParser.json());
const port = 8282;
const results = [
    {
        content: "Hello world!",
        id: 4112,
        time_created: 32313
    }
];

app.get('/v1/questions', (req, res) => {
    res.statusCode = 200;
    res.json(results)
});

app.post('/v1/questions', (req, res) => {
    const questions = req.body;
    console.log(questions);
    console.log(req.json);
    const toAdd = questions.filter(question =>
        results.every(result => result.id !== question.id)
    )
    results.push(...toAdd);
    res.statusCode = 201;
    res.json({})
})

app.listen(port, () => {
    console.log("Listening!")
})