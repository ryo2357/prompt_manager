<script setup lang="ts">
import { reactive, ref } from 'vue'
import type { FormInstance, FormRules } from 'element-plus'
import { invoke } from "@tauri-apps/api/tauri";

const ruleFormRef = ref<FormInstance>()

const submit = (formEl: FormInstance | undefined) => {
  if (!formEl) return
  formEl.validate((valid) => {
    if (valid) {
      // console.log('submit!')
      const prompt = makePrompt()
      invoke('set_clipboard', { text: prompt })
        .then(message => {
          // console.log('クリップボードへの貼り付け成功')
        }).catch(message => {
          // console.error('クリップボードへの転送失敗：', message)
        })
    } else {
      console.log('error submit!')
      return false
    }
  })
}

const resetForm = (formEl: FormInstance | undefined) => {
  if (!formEl) return
  formEl.resetFields()
}

const ruleForm = reactive({
  startPrompt: true,
  title: ``,
  sectionTitle: ``,
  bodyText: ``,
})
const validateTitle = (rule: any, value: any, callback: any) => {
  if (value === '') {
    callback(new Error('論文のタイトルを入力してください'))
  } else {
    callback()
  }
}

const validateSectionTitle = (rule: any, value: any, callback: any) => {
  if (value === '') {
    callback(new Error('セクション名を入力してください'))
  } else {
    callback()
  }
}

const validateBodyText = (rule: any, value: any, callback: any) => {
  if (value === '') {
    callback(new Error('本文を入力してください'))
  } else {
    callback()
  }
}
const rules = reactive<FormRules>({
  title: [{ validator: validateTitle, trigger: 'blur' }],
  sectionTitle: [{ validator: validateSectionTitle, trigger: 'blur' }],
  bodyText: [{ validator: validateBodyText, trigger: 'blur' }],
})

const makePrompt = () => {
  if (ruleForm.startPrompt) {
    return createFirstPrompt(ruleForm.title, ruleForm.sectionTitle, ruleForm.bodyText)
  }
  return createOnceAgain(ruleForm.sectionTitle, ruleForm.bodyText)
}


const createFirstPrompt = (title: string, section_title: string, bodyText: string) => {
  let prompt = `英語の研究論文の一部を日本語で要約するタスクを行います。\n`
  prompt += `これは「${title}」というタイトルの論文の「${section_title}」というセクションの文章です。\n`
  prompt += `以下のルールに従ってください。\n\n`

  prompt += `・リスト形式で出力する (先頭は - を使う)\n`
  prompt += `・簡潔に表現する\n`
  prompt += `・不明な単語や人名と思われるものは英語のまま表示する\n\n`

  prompt += `それでは開始します。\n\n`

  prompt += `英語の論文の一部:\n${bodyText}\n\n`

  prompt += `日本語で要約した文章:`

  return prompt
}

const createOnceAgain = (section_title: string, bodyText: string) => {

  let prompt = `これは先ほど論文の続きで「${section_title}」というセクションの文章です。\n`
  prompt += `英語の論文の一部:\n${bodyText}\n\n`

  prompt += `日本語で要約した文章:`

  return prompt
}

</script>

<template>
  <el-form ref="ruleFormRef" :model="ruleForm" status-icon :rules="rules" label-width="200px" class="demo-ruleForm">

    <el-form-item label="論文のタイトル" prop="title">
      <el-input v-model="ruleForm.title" />
    </el-form-item>


    <el-form-item label="セクションのタイトル" prop="sectionTitle">
      <el-input v-model="ruleForm.sectionTitle" />
    </el-form-item>

    <el-form-item label="本文" prop="bodyText">
      <el-input v-model="ruleForm.bodyText" type="textarea" rows="10" />
    </el-form-item>

    <el-form-item label="初期化プロンプト" prop="startPrompt">
      <el-switch v-model="ruleForm.startPrompt" active-text="含む" inactive-text="含まない" />
    </el-form-item>

    <el-form-item>
      <el-button type="primary" @click="submit(ruleFormRef)">promptの作成</el-button>
      <el-button @click="resetForm(ruleFormRef)">Reset</el-button>
    </el-form-item>

  </el-form>
</template>

<style scoped></style>
