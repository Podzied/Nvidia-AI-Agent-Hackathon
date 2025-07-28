import sys
import json
import os

# --- CONFIGURATION ---
# Mock configuration for testing
MOCK_MODE = os.environ.get("MOCK_MODE", "true").lower() == "true"

# --- INPUT ---
def read_input():
    if not sys.stdin.isatty():
        return json.load(sys.stdin)
    elif len(sys.argv) > 1:
        with open(sys.argv[1], "r") as f:
            return json.load(f)
    else:
        print("Usage: cat compliance.json | python langchain_reasoner_mock.py", file=sys.stderr)
        sys.exit(1)

# --- MOCK LLM CALL ---
def get_reasoning_mock(compliance_result):
    """Mock LLM response for testing without API calls"""
    
    # Extract PII types found
    pii_types = set()
    for detection in compliance_result.get("detected_pii", []):
        pii_types.add(detection.get("pii_type", "Unknown"))
    
    # Generate mock explanation based on detected PII
    if not pii_types:
        explanation = (
            "✅ COMPLIANCE CHECK PASSED\n\n"
            "No PII was detected in the scanned text. The document appears to be compliant "
            "with standard data protection requirements. No further action is required."
        )
    else:
        pii_list = ", ".join(sorted(pii_types))
        explanation = (
            f"⚠️  PII DETECTED: {pii_list}\n\n"
            f"COMPLIANCE ANALYSIS:\n"
            f"- Found {len(compliance_result.get('detected_pii', []))} PII items\n"
            f"- Compliance Score: {compliance_result.get('compliance_score', 0):.1%}\n"
            f"- Redacted Text: {compliance_result.get('redacted_text', 'N/A')}\n\n"
            f"RECOMMENDATIONS:\n"
        )
        
        for rec in compliance_result.get("recommendations", []):
            explanation += f"- {rec}\n"
        
        explanation += (
            "\nNEXT STEPS:\n"
            "1. Review the redacted version for accuracy\n"
            "2. Ensure proper data handling procedures are in place\n"
            "3. Consider implementing automated PII detection in your workflow\n"
            "4. Train staff on data protection best practices"
        )
    
    return explanation

# --- REAL LLM CALL (for when Brev is configured) ---
def get_reasoning_real(compliance_result):
    """Real LLM call using LangChain (requires proper Brev configuration)"""
    try:
        from langchain_community.llms import OpenAI
        from langchain.prompts import PromptTemplate
        
        # Configuration
        BREV_API_URL = os.environ.get("BREV_API_URL", "https://api.brev.dev/v1/completions")
        BREV_API_KEY = os.environ.get("BREV_API_KEY", "YOUR_BREV_API_KEY")
        MODEL_NAME = "llama-3.3-nemotron-super-49b-v1"
        
        # Compose prompt
        prompt = PromptTemplate(
            input_variables=["compliance_json"],
            template=(
                "You are a compliance and security expert. "
                "Given the following compliance scan result (in JSON), explain in plain English what PII was found, why it matters, and what actions should be taken.\n"
                "Compliance Result: {compliance_json}\n"
                "Explanation:"
            ),
        )
        
        # Initialize LLM
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
        
    except Exception as e:
        return f"Error calling LLM: {str(e)}. Using mock response instead."

# --- MAIN ---
def main():
    compliance_result = read_input()
    
    if MOCK_MODE:
        explanation = get_reasoning_mock(compliance_result)
    else:
        explanation = get_reasoning_real(compliance_result)
    
    print(json.dumps({"explanation": explanation}))

if __name__ == "__main__":
    main() 