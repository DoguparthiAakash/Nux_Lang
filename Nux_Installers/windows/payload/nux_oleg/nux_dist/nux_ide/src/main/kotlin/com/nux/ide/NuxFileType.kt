package com.nux.ide

import com.intellij.openapi.fileTypes.LanguageFileType
import javax.swing.Icon

class NuxFileType : LanguageFileType(NuxLanguage.INSTANCE) {
    override fun getName() = "Nux File"
    override fun getDescription() = "Nux language file"
    override fun getDefaultExtension() = "nux"
    override fun getIcon(): Icon? = null // TODO: Add custom Nux Icon
    
    companion object {
        val INSTANCE = NuxFileType()
    }
}
