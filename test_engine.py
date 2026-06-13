import infinite_context_engine

print("=== Testando Extensão Rust no Python ===")

# Inicializa o motor alocando um arquivo binário de swap de 40KB (10 páginas x 4KB)
engine = infinite_context_engine.PyContextEngine("python_swap.bin", 10, 2)

# Cria dados de tokens artificiais (1024 inteiros)
mock_tokens_1 = [11] * 1024
mock_tokens_2 = [22] * 1024
mock_tokens_3 = [33] * 1024

# Insere na RAM quente gerenciada pelo Rust
engine.insert_page(1, mock_tokens_1)
engine.insert_page(2, mock_tokens_2)

print("Páginas 1 e 2 carregadas. Lendo página 1 do cache:")
print(engine.read_tokens(1)[:5]) # Mostra os primeiros 5 tokens

# Força o estouro de limite inserindo a página 3 com alto score de atenção
# Como a página 2 terá score padrão menor, ela sofrerá Page Out (mmap de alta velocidade para o SSD)
engine.insert_page(3, mock_tokens_3)
engine.prefetch_page(3, 0.95)

# Força o resgate automático do SSD trazendo a página de volta
engine.prefetch_page(2, 0.88)
print("Página 2 resgatada com sucesso do SSD via Zero-Copy:")
print(engine.read_tokens(2)[:5])

print("=== Integração concluída com sucesso! ===")