package be.steinsomers.bron_kerbosch

// Source - https://stackoverflow.com/a/79854947

@PublishedApi
internal object Debug {
    @JvmField
    @PublishedApi
    internal val ENABLED: Boolean = javaClass.desiredAssertionStatus()

    inline fun assert(lazyValue: () -> Boolean) {
        if (ENABLED) {
            val value = lazyValue()
            if (!value) {
                throw AssertionError()
            }
        }
    }

    inline fun assert(lazyValue: () -> Boolean, lazyMessage: () -> Any) {
        if (ENABLED) {
            val value = lazyValue()
            if (!value) {
                val message = lazyMessage()
                throw AssertionError(message)
            }
        }
    }
}