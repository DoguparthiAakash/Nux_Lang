import json

with open(r'C:\Users\dogup\.gemini\antigravity-ide\brain\99a9c1db-8c2d-425b-801a-717028fee06b\.system_generated\logs\transcript.jsonl', 'r', encoding='utf-8') as f:
    for line in f:
        data = json.loads(line)
        if data.get('type') == 'TOOL_RESPONSE':
            if 'OP_TENSOR_MATMUL' in str(data.get('content', '')):
                with open('recovered_diff.txt', 'w', encoding='utf-8') as out:
                    out.write(str(data.get('content')))
                print("Recovered to recovered_diff.txt")
