export interface Template {
	id: string;
	name: string;
	category: 'tree' | 'flow' | 'style';
	description: string;
	code: string;
}

export const templates: Template[] = [
	{
		id: 'org-chart',
		name: '组织架构图',
		category: 'tree',
		description: '公司组织架构，展示部门层级关系',
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
		id: 'flow-diagram',
		name: '流程图',
		category: 'flow',
		description: '数据处理流程，从解析到渲染',
		code: `(flow :right
  (start -> parse -> validate)
  (validate -> transform -> render)
  (validate -> error))

(style start :fill "#e3f2fd" :stroke "#1976d2")
(style render :fill "#e8f5e9" :stroke "#388e3c")
(style error :fill "#ffebee" :stroke "#d32f2f")`
	},
	{
		id: 'pipeline',
		name: 'CI/CD 流水线',
		category: 'flow',
		description: '持续集成与部署流水线',
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
		id: 'binary-tree',
		name: '二叉树',
		category: 'tree',
		description: '经典二叉树数据结构',
		code: `(tree :down
  (a :label "1"
    (b :label "2"
      (d :label "4")
      (e :label "5"))
    (c :label "3"
      (f :label "6")
      (g :label "7"))))`
	},
	{
		id: 'decision-tree',
		name: '决策树',
		category: 'tree',
		description: '简单的决策流程树',
		code: `(tree :down
  (start :label "开始"
    (check :label "条件判断"
      (yes :label "是 → 执行A")
      (no :label "否 → 执行B"))))

(style start :fill "#e3f2fd" :stroke "#1976d2")
(style check :fill "#fff3e0" :stroke "#ff9800")
(style yes :fill "#e8f5e9" :stroke "#4caf50")
(style no :fill "#ffebee" :stroke "#d32f2f")`
	},
	{
		id: 'microservices',
		name: '微服务架构',
		category: 'flow',
		description: '微服务间的调用关系',
		code: `(flow :right
  (gateway :label "API Gateway" -> auth :label "Auth")
  (gateway -> users :label "Users")
  (gateway -> orders :label "Orders")
  (orders -> payment :label "Payment")
  (orders -> inventory :label "Inventory"))

(style gateway :fill "#e8eaf6" :stroke "#3f51b5")
(style auth :fill "#fce4ec" :stroke "#e91e63")
(style payment :fill "#e8f5e9" :stroke "#4caf50")
(style inventory :fill "#fff3e0" :stroke "#ff9800")`
	},
	{
		id: 'define-template',
		name: '模板复用',
		category: 'tree',
		description: '用 define 定义可复用组件，实例化多个副本并连线',
		code: `(define server (params name)
  (cpu :label "\${name} CPU")
  (eth0)
  (eth1))

(tree :down
  (rack :label "机架"
    (server s1 "S1")
    (server s2 "S2")))

(line :straight s1.eth0 -> s2.eth0 :desc "网络连接")

(style s1.cpu :fill "#e8f4fd" :stroke "#2196f3")
(style s2.cpu :fill "#fff3e0" :stroke "#ff9800")`
	},
	{
		id: 'styled-tree',
		name: '彩色样式树',
		category: 'style',
		description: '展示丰富的样式配置',
		code: `(tree :down
  (root :label "根节点"
    (a :label "节点 A"
      (a1 :label "A-1")
      (a2 :label "A-2"))
    (b :label "节点 B"
      (b1 :label "B-1"))))

(style root :fill "#1a237e" :stroke "#283593" :color "#ffffff")
(style a :fill "#4a148c" :stroke "#6a1b9a" :color "#ffffff")
(style b :fill "#004d40" :stroke "#00695c" :color "#ffffff")
(style a1 :fill "#e8eaf6" :stroke "#3f51b5")
(style a2 :fill "#f3e5f5" :stroke "#9c27b0")
(style b1 :fill "#e0f2f1" :stroke "#009688")`
	}
];

export const categories = [
	{ id: 'all', label: '全部' },
	{ id: 'tree', label: '树形图' },
	{ id: 'flow', label: '流程图' },
	{ id: 'style', label: '样式示例' }
] as const;
