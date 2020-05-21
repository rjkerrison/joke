pub enum Utterance {
    Normal(String),
    Quote(Quote),
}

pub enum Quote {
    SurpriseAtAnIntruder(String),
    PertinentToAlgorithms(String),
}

impl Quote {
    fn is_relevant_to_conversation(&self) -> bool {
        match self {
            Quote::PertinentToAlgorithms(_) => true,
            Quote::SurpriseAtAnIntruder(_) => false,
        }
    }
}

pub struct Conversation {
    utterances: Vec<Utterance>,
}

impl Conversation {
    fn everything_is_relevant_and_normal(&self) -> bool {
        let weird_utterances: Vec<&Utterance> = self
            .utterances
            .iter()
            .filter(|utterance| match utterance {
                Utterance::Quote(quote) => !quote.is_relevant_to_conversation(),
                Utterance::Normal(_) => false,
            })
            .collect();

        weird_utterances.len() == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    fn build_conversation_with_quote(quote: Quote) -> Conversation {
        Conversation {
            utterances: vec![
                Utterance::Normal("Man, you're being inconsistent with your array indices. Some are from one, some are from zero.".into()),
                Utterance::Normal("Different algorithms call for different conventions. To quote Stanford algorithms professor Donald Knuth…".into()),
                Utterance::Quote(quote)
            ],
        }
    }

    #[test]
    fn test_surprise_at_intruder_quote_is_not_relevant() {
        let quote =
            Quote::SurpriseAtAnIntruder("Who are you? How did you get into my house?".to_string());

        let conversation = build_conversation_with_quote(quote);
        assert_eq!(false, conversation.everything_is_relevant_and_normal());
    }

    #[test]
    fn test_algorithms_quote_is_relevant() {
        let quote = Quote::PertinentToAlgorithms(
            "Sometimes you should use zero-based indices; sometimes, use one-based.".to_string(),
        );

        let conversation = build_conversation_with_quote(quote);
        assert_eq!(false, conversation.everything_is_relevant_and_normal());
    }
}
