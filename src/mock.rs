use std::path::PathBuf;

use subparse::SubtitleEntry;

use crate::{
    formatter::Formatter, parse::parse_file, subtitle_extractor::SubtitleExtractor,
    text_splitter::TextSplitter, translator::Translator,
};

pub struct Mock {
    pub path: PathBuf,
    pub subtitle_entries: Vec<SubtitleEntry>,
    pub subtitle_extractor: SubtitleExtractor,
    pub text_splitter: TextSplitter,
    pub formatter: Formatter,
    pub translator: Translator,
}

impl Mock {
    pub fn new() -> Self {
        let path = PathBuf::from("test.srt");
        let subtitle_entries = parse_file(&path);
        let subtitle_extractor = SubtitleExtractor::extractor(&subtitle_entries);
        let text_splitter = TextSplitter::split_text(&subtitle_extractor.text_info);
        let formatter = crate::formatter::Formatter::format(1, &text_splitter.split_result);
        let mut translator = Translator::new();
        translator.translated_result = String::from("【思考】第一轮直译主要是保留原文的字面意思，不遗漏任何信息。\n\n【翻译】\n```\n你好！<T>是我，<T>灵魂的召唤者，<nl>黑暗艺术的主人！<T>有人在吗？<T>是我，梅林。<T>那位巫师？<T>记得我吗？<T>不！我保守了你的秘密！<nl>确实！就像我承诺的那样！<T>我没有告诉任何人你的存在。没有人！<T>但你必须理解，我们不列颠人<T>正在进行一场绝望的斗争。<T>末日般的事情。我是认真的。<T>现在它正在那里发生<nl>就在我们说话的时候。<T>太可怕了！<T>大人物们在<nl>冲突，血腥交加。<T>我不想请求……但我们需要你的帮助。<T>没有咒语。没有魔法。<T>- 我们撤退，活着再战。<nl>- 不！<T>没有牺牲……<nl>就没有胜利。<T>这真是疯狂！\n```\n\n【思考】第二轮意译在直译的基础上对文本进行更加通俗流畅的表达，确保信息的完整性。\n\n【翻译】\n```\n你好！<T>我就是，<T>灵魂的召唤者，<nl>黑暗艺术的大师！<T>有人在吗？<T>我就是梅林。<T>那位巫师？<T>你记得我吗？<T>不！我一直在保守你的秘密！<nl>我真的保守了！就像我答应过你的！<T>我没跟任何人提起过你的存在。没人！<T>但你要明白，我们不列颠人<T>正在进行一场绝望的斗争。<T>这是末世的征兆。我是认真的。<T>这一切正在那里发生<nl>就在我们说话的此刻。<T>太可怕了！<T>一些大人物们在<nl>冲突，血流成河。<T>我实在不想请求……但我们真的需要你的帮助。<T>没有咒语。没有魔法。<T>- 我们撤退，活着再战。<nl>- 不！<T>没有牺牲……<nl>就没有胜利。<T>这太疯狂了！\n```\n\n【思考】第三轮反思环节将对译文进行详细审视，提出建设性批评和建议。\n\n*准确性*; 部分短句可以更简洁表达，比如“我就是梅林”可以直接称呼，避免重复。“我真的保守了！就像我答应过你的！”可优化。 \n*流畅性*; 某些地方的连接稍显生硬，例如“这是末世的征兆。我是认真的。”可以合并成一句，增加语句的连贯性。\n*风格*; “这太疯狂了！”可以换成更口语化的说法，例如“这简直是疯了！”。\n*术语*; “黑暗艺术的主人”可改为“黑暗艺术的高手”，更符合大众观看习惯，减少生硬感。\n\n【建议】\n```\n1. 减少短句重复，提升简洁性。\n2. 增强语句间的连贯性，可以合并一些短句。\n3. 使用更口语化的表达，增加亲和力。\n4. 确保术语的流行用法，提升整体可读性。\n```\n\n【思考】在提升环节，我将根据建议与反馈对翻译进行完善，保证翻译更加流畅和符合观众习惯。\n\n```\n你好！<T>我就是，<T>灵魂的召唤者，<nl>黑暗艺术的高手！<T>有人在吗？<T>我就是梅林。<T>那位巫师？<T>记得我吗？<T>不！我一直在保守你的秘密！<nl>我真的保守了，跟你承诺的一样！<T>我没告诉任何人你的存在。没人！<T>但你得明白，我们不列颠人<T>正在拼命抗争。<T>这是末世的征兆，我是认真的。<T>这一切正在那里发生<nl>就在我们说话的此刻。<T>太可怕了！<T>那些大人物们在<nl>冲突，血流成河。<T>我不想请求……但我们真的需要你的帮助。<T>没有咒语，没有魔法。<T>- 我们撤退，活着再战。<nl>- 不！<T>没有牺牲……<nl>就没有胜利。<T>这简直是疯了！\n```");
        let translator = translator.format_translated_result();
        Self {
            path,
            subtitle_entries,
            subtitle_extractor,
            text_splitter,
            formatter,
            translator,
        }
    }
}
