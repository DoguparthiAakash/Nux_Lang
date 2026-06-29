with open('src/compiler.rs', 'r') as f:
    lines = f.readlines()

start = [i for i,l in enumerate(lines) if 'fn parse_statement_impl' in l][0]
end = [i for i,l in enumerate(lines) if 'fn parse_statement_or_expr' in l][0]

print(f"{start} to {end}")
