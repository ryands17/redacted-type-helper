# Redacted type helper

This is a simple project showcasing the redacted type helper that can be helpful to prevent PII to be logged to any system. Annotating a field with the `Redacted` type will make sure that it is displayed as `<Redacted>` everywhere it is being logged.

You can still extract the inner value when using the debug print mechanism and also by accessing the inner struct value.
