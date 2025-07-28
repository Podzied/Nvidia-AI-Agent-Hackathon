import sys
import json
import os
from langchain_community.llms import OpenAI
from langchain.prompts import PromptTemplate

# --- CONFIGURATION ---
# Replace with your actual Brev endpoint and API key
BREV_API_URL = os.environ.get("BREV_API_URL", "https://api.brev.dev/v1/completions")
BREV_API_KEY = os.environ.get("BREV_API_KEY", "YOUR_BREV_API_KEY")
MODEL_NAME = "llama-3.3-nemotron-super-49b-v1"

# --- INPUT ---
def read_input():
    if not sys.stdin.isatty():
        return json.load(sys.stdin)
    elif len(sys.argv) > 1:
        with open(sys.argv[1], "r") as f:
            return json.load(f)
    else:
        print("Usage: cat compliance.json | python langchain_reasoner.py", file=sys.stderr)
        sys.exit(1)

# --- LLM CALL ---
def get_reasoning(compliance_result):
    # Compose a prompt for reasoning
    prompt = PromptTemplate(
        input_variables=["compliance_json"],
        template=(
            "You are a compliance and security expert. "
            "Given the following compliance scan result (in JSON), explain in plain English what PII was found, why it matters, and what actions should be taken.\n"
            "Compliance Result: {compliance_json}\n"
            "Explanation:"
        ),
    )
    
    # Use LangChain's OpenAI LLM wrapper with custom endpoint (Brev)
    llm = OpenAI(
        openai_api_base=BREV_API_URL,
        openai_api_key=BREV_API_KEY,
        model_name=MODEL_NAME,
        temperature=0.2,
        max_tokens=512,
    )
    
    prompt_str = prompt.format(compliance_json=json.dumps(compliance_result, indent=2))
    response = llm.invoke(prompt_str)
    return response

# --- MAIN ---
def main():
    compliance_result = read_input()
    explanation = get_reasoning(compliance_result)
    print(json.dumps({"explanation": explanation}))

if __name__ == "__main__":
    main() 