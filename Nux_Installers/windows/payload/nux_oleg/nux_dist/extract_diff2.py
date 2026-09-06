import json

with open(r'C:\Users\dogup\.gemini\antigravity-ide\brain\99a9c1db-8c2d-425b-801a-717028fee06b\.system_generated\logs\transcript.jsonl', 'r', encoding='utf-8') as f:
    for line in f:
        data = json.loads(line)
        if data.get('type') == 'TOOL_RESPONSE':
            output = str(data.get('content'))
            if 'OP_TENSOR_MATMUL' in output or 'OP_TENSOR_RELU' in output:
                with open('recovered_diff.txt', 'a', encoding='utf-8') as out:
                    out.write(output + '\n---\n')
