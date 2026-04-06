export interface Example {
	name: string;
	code: string;
}

export const examples: Example[] = [
	{
		name: 'Org Chart',
		code: `(tree :down
  (ceo :label "CEO"
    (dev :label "研发部"
      (fe :label "前端")
      (be :label "后端")
      (qa :label "测试"))
    (pm :label "产品部")
    (hr :label "人力资源")))

(line :straight fe -> be :desc "协作" :color "#4a90d9")

(style ceo :fill "#e8f4fd" :stroke "#2196f3")
(style dev :fill "#e8f5e9" :stroke "#4caf50")
(style pm :fill "#fff3e0" :stroke "#ff9800")
(style hr :fill "#fce4ec" :stroke "#e91e63")`
	},
	{
		name: 'Flow Diagram',
		code: `(flow :right
  (start -> parse -> validate)
  (validate -> transform -> render)
  (validate -> error))

(style start :fill "#e3f2fd" :stroke "#1976d2")
(style render :fill "#e8f5e9" :stroke "#388e3c")
(style error :fill "#ffebee" :stroke "#d32f2f")`
	},
	{
		name: 'Pipeline',
		code: `(flow :right
  (src :label "Source" -> build :label "Build" -> test :label "Test")
  (test -> deploy :label "Deploy" -> monitor :label "Monitor")
  (test -> fix :label "Fix" -> build))

(line monitor -> src :desc "Feedback")

(style src :fill "#e8eaf6" :stroke "#3f51b5")
(style deploy :fill "#e8f5e9" :stroke "#4caf50")
(style fix :fill "#fff8e1" :stroke "#f9a825")`
	},
	{
		name: 'Binary Tree',
		code: `(tree :down
  (a :label "1"
    (b :label "2"
      (d :label "4")
      (e :label "5"))
    (c :label "3"
      (f :label "6")
      (g :label "7"))))`
	}
];
