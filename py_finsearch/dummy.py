from openai import OpenAI




if __name__ == "__main__":
    client = OpenAI(api_key="NONE", base_url="https://alpha---finsearch-l35slsdlnq-uc.a.run.app")

    chat_completion = client.chat.completions.create(
        messages=[
        {"role": "system", "content": "You are a helpful assistant."},
        {"role": "user", "content": "What is the top news about Gen AI today? "}
        ],
        model = "WHATEVER_MODEL"
    )

    print(chat_completion.choices[0].content)
