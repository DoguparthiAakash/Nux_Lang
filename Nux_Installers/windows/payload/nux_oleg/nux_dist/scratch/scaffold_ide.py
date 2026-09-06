import os
import pathlib

base_dir = pathlib.Path("nux_ide")
base_dir.mkdir(exist_ok=True)

# settings.gradle.kts
with open(base_dir / "settings.gradle.kts", "w", encoding="utf-8") as f:
    f.write('rootProject.name = "NuxIDE"\n')

# build.gradle.kts
with open(base_dir / "build.gradle.kts", "w", encoding="utf-8") as f:
    f.write('''
plugins {
    id("java")
    id("org.jetbrains.kotlin.jvm") version "1.9.0"
    id("org.jetbrains.intellij") version "1.15.0"
}

group = "com.nux.ide"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

// Configure Gradle IntelliJ Plugin
intellij {
    version.set("2023.1.5")
    type.set("IC") // IntelliJ IDEA Community Edition
}

tasks {
    withType<JavaCompile> {
        sourceCompatibility = "17"
        targetCompatibility = "17"
    }
    withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
        kotlinOptions.jvmTarget = "17"
    }
    patchPluginXml {
        sinceBuild.set("223")
        untilBuild.set("242.*")
    }
    signPlugin {
        certificateChain.set(System.getenv("CERTIFICATE_CHAIN"))
        privateKey.set(System.getenv("PRIVATE_KEY"))
        password.set(System.getenv("PRIVATE_KEY_PASSWORD"))
    }
    publishPlugin {
        token.set(System.getenv("PUBLISH_TOKEN"))
    }
}
''')

# Create directories
src_dir = base_dir / "src" / "main" / "kotlin" / "com" / "nux" / "ide"
src_dir.mkdir(parents=True, exist_ok=True)

res_dir = base_dir / "src" / "main" / "resources" / "META-INF"
res_dir.mkdir(parents=True, exist_ok=True)

# plugin.xml
with open(res_dir / "plugin.xml", "w", encoding="utf-8") as f:
    f.write('''<idea-plugin>
    <id>com.nux.ide</id>
    <name>Nux IDE</name>
    <vendor email="support@nuxlang.org" url="http://nuxlang.org">Nux Foundation</vendor>

    <description><![CDATA[
    A beautiful, unique IDE for the Nux programming language. <br>
    Built on the JetBrains open-source platform, providing advanced quantum and classical language features.
    ]]></description>

    <depends>com.intellij.modules.platform</depends>

    <extensions defaultExtensionNs="com.intellij">
        <fileType name="Nux File" implementationClass="com.nux.ide.NuxFileType" fieldName="INSTANCE" language="Nux" extensions="nux"/>
        <!-- <syntaxHighlighter key="Nux" implementationClass="com.nux.ide.NuxSyntaxHighlighter"/> -->
        <colorSettingsPage implementation="com.nux.ide.NuxColorSettingsPage"/>
    </extensions>
</idea-plugin>
''')

# NuxLanguage.kt
with open(src_dir / "NuxLanguage.kt", "w", encoding="utf-8") as f:
    f.write('''package com.nux.ide

import com.intellij.lang.Language

class NuxLanguage : Language("Nux") {
    companion object {
        val INSTANCE = NuxLanguage()
    }
}
''')

# NuxFileType.kt
with open(src_dir / "NuxFileType.kt", "w", encoding="utf-8") as f:
    f.write('''package com.nux.ide

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
''')

# NuxColorSettingsPage.kt
with open(src_dir / "NuxColorSettingsPage.kt", "w", encoding="utf-8") as f:
    f.write('''package com.nux.ide

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
''')

print("Nux IDE IntelliJ Plugin scaffolded successfully.")
