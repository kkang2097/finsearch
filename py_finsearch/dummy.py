from openai import OpenAI




if __name__ == "__main__":
    client = OpenAI(api_key="NONE", base_url="http://localhost:7878")

    chat_completion = client.chat.completions.create(
        messages = [
            {
                "role": "user",
                "content": "What's the latest news on Apple M4 chips?"
            }
        ],
        model = "WHATEVER_MODEL"
    )

    print(chat_completion.choices[0].content)
