package com.nux.ide

import com.intellij.lang.Language

class NuxLanguage : Language("Nux") {
    companion object {
        val INSTANCE = NuxLanguage()
    }
}
