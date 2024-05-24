package com.tarikeshaq.conferenceapp

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import androidx.lifecycle.viewmodel.compose.viewModel
import com.tarikeshaq.conferenceapp.ui.theme.ConferenceAppTheme
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import uniffi.conference.QuestionRecord
import uniffi.conference.QaManager
import uniffi.conference.HttpClient
import java.io.BufferedReader
import java.io.InputStreamReader
import java.io.OutputStreamWriter
import java.net.HttpURLConnection
import java.net.URL

const val BASE_URL = "http://10.0.2.2:8282/v1/questions"

@Serializable
data class RemoteQuestion(
    val content: String,
    val id: ULong,
    val time_created: ULong
)

fun RemoteQuestion.into(): QuestionRecord {
    return QuestionRecord(
        content = this.content,
        id = this.id,
        timeCreated = this.time_created
    )
}

fun QuestionRecord.into(): RemoteQuestion {
    return RemoteQuestion(
        content = this.content,
        id = this.id,
        time_created = this.timeCreated
    )
}

class KotlinHttpClient: HttpClient {
    override suspend fun write(values: List<QuestionRecord>) = withContext(Dispatchers.IO) {
        val urlObj = URL(BASE_URL)
        val connection = urlObj.openConnection() as HttpURLConnection
        connection.requestMethod = "POST"
        connection.doOutput = true
        connection.setRequestProperty("Content-Type", "application/json")

        try {
            val outputStream = OutputStreamWriter(connection.outputStream)
            outputStream.write(Json.encodeToString(values.map { it.into() }))
            outputStream.flush()
            outputStream.close()

            val responseCode = connection.responseCode
            if (responseCode == HttpURLConnection.HTTP_OK || responseCode == HttpURLConnection.HTTP_CREATED) {
                val inputStream = connection.inputStream
                val reader = BufferedReader(InputStreamReader(inputStream))
                val response = StringBuilder()
                var line: String?

                while (reader.readLine().also { line = it } != null) {
                    response.append(line)
                }

                reader.close()
            } else {
                throw Exception("HTTP POST Request Failed with Error code: $responseCode")
            }
        } finally {
            connection.disconnect()
        }
    }

    override suspend fun get(): List<QuestionRecord> = withContext(Dispatchers.IO) {
        val urlObj = URL(BASE_URL)
        val connection = urlObj.openConnection() as HttpURLConnection
        connection.requestMethod = "GET"

        try {
            val responseCode = connection.responseCode
            if (responseCode == HttpURLConnection.HTTP_OK) {
                val inputStream = connection.inputStream
                val reader = BufferedReader(InputStreamReader(inputStream))
                val response = StringBuilder()
                var line: String?

                while (reader.readLine().also { line = it } != null) {
                    response.append(line)
                }

                reader.close()
                val res = Json.decodeFromString<List<RemoteQuestion>>(response.toString())
                res.map {
                    it.into()
                }
            } else {
                throw Exception("HTTP GET Request Failed with Error code: $responseCode")
            }
        } finally {
            connection.disconnect()
        }
    }
}

class MainViewModel : ViewModel() {
    private val _items = MutableStateFlow<List<String>>(emptyList())
    val items: StateFlow<List<String>> = _items

    fun syncWithServer() {
        viewModelScope.launch {
            manager.sync()
            _items.value = manager.get()
        }
    }

    fun processNewQuestion(input: String) {
        viewModelScope.launch {
            manager.write(value = input)
            _items.value = manager.get()
        }
    }
}

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            ConferenceAppTheme {
                Surface(color = MaterialTheme.colorScheme.background) {
                    Greeting()
                }
            }
        }
    }
}
val manager: QaManager = QaManager(KotlinHttpClient())

@Composable
fun Greeting(mainViewModel: MainViewModel = viewModel()) {
    val items by mainViewModel.items.collectAsState()
    var text by remember { mutableStateOf("") }

    Column(modifier = Modifier
        .fillMaxSize()
        .padding(16.dp)) {
        LazyColumn(modifier = Modifier.weight(1f)) {
            items(items) { item ->
                Text(text = item, modifier = Modifier.padding(8.dp))
            }
        }
        // Text input and submit button
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween
        ) {
            TextField(
                value = text,
                onValueChange = { text = it },
                modifier = Modifier.weight(1f),
                placeholder = { Text("Enter text") }
            )
            Button(
                onClick = {
                    mainViewModel.processNewQuestion(text)
                    text = ""
                },
                modifier = Modifier.padding(start = 8.dp)
            ) {
                Text("Submit")
            }
        }
        Button(
            onClick = { mainViewModel.syncWithServer() },
            modifier = Modifier.fillMaxWidth()
        ) {
            Text(text = "Sync with server")
        }
    }
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    ConferenceAppTheme {
        Greeting()
    }
}