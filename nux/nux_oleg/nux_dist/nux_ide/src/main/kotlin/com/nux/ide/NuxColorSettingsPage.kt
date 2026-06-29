package com.nux.ide

import com.intellij.openapi.options.colors.ColorDescriptor
import com.intellij.openapi.options.colors.ColorSettingsPage
import com.intellij.openapi.editor.colors.TextAttributesKey
import com.intellij.openapi.fileTypes.SyntaxHighlighter
import com.intellij.openapi.fileTypes.PlainSyntaxHighlighter
import javax.swing.Icon

class NuxColorSettingsPage : ColorSettingsPage {
    override fun getIcon(): Icon? = null
    override fun getHighlighter(): SyntaxHighlighter = PlainSyntaxHighlighter()
    override fun getDemoText() = """
// Welcome to Nux IDE
fn main() {
    let qreg = create_qreg(2);
    qreg.entangle(0, 1);
    print(qreg.measure(0));
}
    """.trimIndent()
    override fun getAdditionalHighlightingTagToDescriptorMap(): Map<String, TextAttributesKey>? = null
    override fun getAttributeDescriptors(): Array<com.intellij.openapi.options.colors.AttributesDescriptor> = emptyArray()
    override fun getColorDescriptors(): Array<ColorDescriptor> = ColorDescriptor.EMPTY_ARRAY
    override fun getDisplayName() = "Nux"
}
