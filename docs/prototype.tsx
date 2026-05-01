import React, { useState, useEffect, useMemo, useRef } from 'react';
import { 
  Play, ChevronRight, ChevronLeft, BookOpen, CheckCircle2, 
  HelpCircle, Info, X, RotateCcw, LayoutGrid, Search, 
  Clock, Brain, Target, BarChart3, Star, Zap, Layers,
  MessageSquare, History, Bookmark, Filter, Award, 
  ChevronDown, GraduationCap, Flame
} from 'lucide-react';

/**
 * --------------------------------------------------------------------------------
 * 1. Design System & Constants
 * --------------------------------------------------------------------------------
 */
const COLORS = {
  bgBase: '#121212',
  bgSurface: '#181818',
  bgSurfaceElevated: '#242424',
  spotifyGreen: '#1ed760',
  textSecondary: '#b3b3b3',
  pill: '9999px',
};

// 重新设计数据结构：支持 游戏仓库(Library) -> 单个游戏(Course) -> 章节(L1~LN) -> 步骤(Step) 的层级
const MOCK_GAMES = [
  { 
    id: 'G1', 
    title: '量化交易：从零到一', 
    category: '编程与金融', 
    color: '#1ed760', 
    mastery: 45, 
    itemsToReview: 17,
    chapters: [
      { id: 'L1', title: 'Python 自动化入门', progress: 100, steps: [{sequence_id: 'step1', concept: '核心定义'}, {sequence_id: 'step2', concept: '环境配置'}] },
      { id: 'L2', title: 'Web Scraping 进阶', progress: 60, steps: [{sequence_id: 'step1', concept: 'DOM 树解析'}, {sequence_id: 'step2', concept: 'Anti-bot 策略'}, {sequence_id: 'step3', concept: '互动小测'}] },
      { id: 'L3', title: '算法交易策略', progress: 0, steps: [{sequence_id: 'step1', concept: '均值回归'}] },
    ]
  },
  { 
    id: 'G2', 
    title: '全栈 Web3 开发', 
    category: '区块链', 
    color: '#539df5', 
    mastery: 10, 
    itemsToReview: 0,
    chapters: [
      { id: 'L1', title: '以太坊与智能合约', progress: 10, steps: [{sequence_id: 'step1', concept: 'EVM 基础'}] }
    ]
  }
];

/**
 * --------------------------------------------------------------------------------
 * 2. Helper Components
 * --------------------------------------------------------------------------------
 */

const PillButton = ({ children, onClick, variant = 'primary', className = '', icon: Icon, disabled }) => {
  const baseStyle = "px-6 py-3 rounded-full font-bold uppercase tracking-wider text-[11px] transition-all flex items-center justify-center gap-2 disabled:opacity-30 whitespace-nowrap active:scale-95";
  const variants = {
    primary: "bg-[#1ed760] text-black hover:scale-105",
    secondary: "bg-white text-black hover:scale-105",
    outline: "bg-transparent text-white border border-[#727272] hover:border-white",
    ghost: "bg-transparent text-[#b3b3b3] hover:text-white",
    danger: "bg-[#f3727f] text-white",
  };
  return (
    <button onClick={onClick} disabled={disabled} className={`${baseStyle} ${variants[variant]} ${className}`}>
      {Icon && <Icon size={14} />}
      {children}
    </button>
  );
};

/**
 * --------------------------------------------------------------------------------
 * 3. Galgame Narrative Player (Guided Story Mode)
 * --------------------------------------------------------------------------------
 */
function GalgamePlayer({ game, chapter, stepData, onExit, setSelectedTerm }) {
  const [screenIdx, setScreenIdx] = useState(0);
  const [showLog, setShowLog] = useState(false);
  const currentScreen = stepData.screens[screenIdx];
  const progress = ((screenIdx + 1) / stepData.screens.length) * 100;
  
  const history = useMemo(() => stepData.screens.slice(0, screenIdx), [screenIdx, stepData]);

  const handleNext = () => {
    if (screenIdx < stepData.screens.length - 1) setScreenIdx(v => v + 1);
    else onExit();
  };

  const handlePrev = () => {
    if (screenIdx > 0) setScreenIdx(v => v - 1);
  };

  const parseLines = (text) => {
    if (!text) return null;
    const parts = text.split(/(<term id="[^"]+">.*?<\/term>)/g);
    return parts.map((part, index) => {
      const match = part.match(/<term id="([^"]+)">([\s\S]*?)<\/term>/);
      if (match) {
        const id = match[1];
        const label = match[2];
        return (
          <span
            key={index}
            onClick={(e) => { 
              e.stopPropagation(); 
              setSelectedTerm({ id, ...stepData.term_catalog[id] }); 
            }}
            className="cursor-help text-[#1ed760] border-b border-dotted border-[#1ed760] hover:bg-[#1ed760]/10 px-0.5 rounded transition-all font-bold"
          >
            {label}
          </span>
        );
      }
      return part;
    });
  };

  return (
    <div className="fixed inset-0 z-[100] bg-[#080808] flex flex-col items-center justify-center animate-in fade-in duration-500">
      {/* 沉浸式背景层 */}
      <div className={`absolute inset-0 transition-all duration-1000 ${currentScreen.type === 'exercise' ? 'bg-[#1ed760]/5' : 'bg-black'}`}>
         <div className="absolute inset-0 opacity-10 bg-[radial-gradient(circle_at_center,_var(--tw-gradient-stops))] from-[#1ed760] via-transparent to-transparent" />
      </div>

      {/* 顶部控制栏 */}
      <div className="absolute top-0 left-0 right-0 p-8 flex justify-between items-start z-10">
        <div className="flex items-center gap-4">
           <div className="w-10 h-10 bg-[#1ed760] rounded-lg flex items-center justify-center text-black font-black text-xs text-center leading-tight">
              {chapter.id}
           </div>
           <div>
              <p className="text-[10px] font-bold text-[#1ed760] uppercase tracking-[0.2em] mb-0.5">{game.title}</p>
              <h2 className="text-sm font-bold text-white/60 tracking-tight">{chapter.title} • {stepData.sequence_id.toUpperCase()}</h2>
           </div>
        </div>
        <div className="flex gap-2">
           <button onClick={() => setShowLog(true)} className="p-3 bg-white/5 hover:bg-white/10 rounded-full text-white/50 hover:text-white transition-all">
              <History size={20} />
           </button>
           <button onClick={onExit} className="p-3 bg-white/5 hover:bg-white/10 rounded-full text-white/50 hover:text-white transition-all">
              <X size={20} />
           </button>
        </div>
      </div>

      {/* 视觉内容区 */}
      <div className="w-full max-w-4xl px-8 flex flex-col items-center gap-12 relative z-10 transition-all">
        {currentScreen.type === 'exercise' ? (
          <div className="w-full max-w-xl bg-[#181818] p-10 rounded-3xl border border-white/5 shadow-[0_32px_64px_-12px_rgba(0,0,0,0.8)] animate-in zoom-in-95 duration-500">
             <div className="flex items-center gap-2 text-blue-400 mb-6 font-bold uppercase tracking-widest text-[10px]">
                <Target size={14} /> 交互式检测
             </div>
             <p className="text-xl font-bold mb-8 leading-tight text-white/90">{currentScreen.exercise.prompt}</p>
             <div className="grid gap-3">
                {currentScreen.exercise.options.map((opt, i) => (
                  <button key={i} className="text-left p-4 rounded-xl border border-white/5 hover:border-[#1ed760] hover:bg-[#1ed760]/5 transition-all text-white/60 hover:text-white text-sm">
                    {opt}
                  </button>
                ))}
             </div>
          </div>
        ) : (
          <div className="w-48 h-48 rounded-full bg-gradient-to-b from-[#1ed760]/10 to-transparent border border-white/5 flex items-center justify-center animate-pulse">
             <GraduationCap size={64} className="text-[#1ed760]/30" />
          </div>
        )}
      </div>

      {/* Galgame 对话框布局 */}
      <div className="absolute bottom-0 left-0 right-0 p-6 md:p-12 flex justify-center">
        <div 
          onClick={handleNext}
          className="w-full max-w-5xl bg-[#121212]/95 backdrop-blur-3xl border border-white/10 rounded-[2.5rem] p-10 relative group cursor-pointer shadow-2xl hover:bg-[#181818]/95 transition-all"
        >
          {/* 对话框顶部的名字标签 */}
          <div className="absolute -top-4 left-12 bg-[#1ed760] text-black px-8 py-2 rounded-full text-[11px] font-black uppercase tracking-[0.2em] shadow-lg">
             {currentScreen.type === 'exercise' ? 'System Test' : 'Instructor'}
          </div>

          {/* 进度微型条 */}
          <div className="absolute top-0 left-12 right-12 h-[2px] bg-white/5 overflow-hidden rounded-full">
             <div className="h-full bg-[#1ed760] transition-all duration-700" style={{ width: `${progress}%` }} />
          </div>

          {/* 文字叙事 */}
          <div className="min-h-[140px] flex flex-col justify-center gap-4">
            {currentScreen.lines.map((line, idx) => (
              <p key={idx} className="text-2xl md:text-3xl font-bold leading-relaxed text-white/90 tracking-tight">
                {parseLines(line)}
              </p>
            ))}
          </div>

          {/* 辅助操作栏 */}
          <div className="mt-8 flex items-center justify-between opacity-40 group-hover:opacity-100 transition-opacity">
             <div className="flex gap-4">
                <button 
                  onClick={(e) => { e.stopPropagation(); handlePrev(); }}
                  disabled={screenIdx === 0}
                  className="flex items-center gap-1 text-[10px] font-bold uppercase tracking-widest hover:text-[#1ed760] disabled:opacity-0"
                >
                  <ChevronLeft size={16} /> Back
                </button>
                <button className="text-[10px] font-bold uppercase tracking-widest hover:text-[#1ed760] flex items-center gap-1">
                  <Bookmark size={14} /> Quick Save
                </button>
             </div>
             <div className="flex items-center gap-2 text-[10px] font-bold uppercase tracking-[0.3em] group-hover:text-[#1ed760] animate-pulse">
                Click to Continue <ChevronRight size={16} />
             </div>
          </div>
        </div>
      </div>

      {/* 对话日志侧边栏 */}
      {showLog && (
        <div className="fixed inset-0 z-[110] bg-black/80 backdrop-blur-md flex justify-end animate-in fade-in duration-300">
           <div className="w-full max-w-md bg-[#121212] h-full p-8 flex flex-col border-l border-white/10 animate-in slide-in-from-right duration-500">
              <div className="flex justify-between items-center mb-8">
                 <h3 className="text-xl font-black uppercase tracking-tighter flex items-center gap-2"><History /> Narrative Log</h3>
                 <button onClick={() => setShowLog(false)} className="text-[#b3b3b3] hover:text-white"><X /></button>
              </div>
              <div className="flex-1 overflow-y-auto space-y-8 pr-4">
                 {history.length === 0 ? <p className="text-[#333] italic">No logs yet.</p> : history.map((h, i) => (
                   <div key={i} className="space-y-2 opacity-60 hover:opacity-100 transition-opacity">
                      <p className="text-[10px] font-bold text-[#1ed760] uppercase">Page {i + 1}</p>
                      <p className="text-white font-medium leading-relaxed">{h.lines.join(' ')}</p>
                   </div>
                 ))}
              </div>
           </div>
        </div>
      )}
    </div>
  );
}

/**
 * --------------------------------------------------------------------------------
 * 4. FSRS Review Center
 * --------------------------------------------------------------------------------
 */
function FSRSReviewCenter({ cards, onComplete, gameTitle }) {
  const [idx, setIdx] = useState(0);
  const [flipped, setFlipped] = useState(false);
  const currentCard = cards[idx];

  const handleRating = (rating) => {
    setFlipped(false);
    if (idx < cards.length - 1) setIdx(v => v + 1);
    else onComplete();
  };

  return (
    <div className="max-w-2xl mx-auto py-12 animate-in fade-in duration-700">
       <header className="text-center mb-16">
          <div className="flex justify-center gap-2 mb-4">
             {cards.map((_, i) => (
               <div key={i} className={`h-1 rounded-full transition-all duration-500 ${i === idx ? 'w-8 bg-[#1ed760]' : i < idx ? 'w-4 bg-[#1ed760]/30' : 'w-4 bg-white/10'}`} />
             ))}
          </div>
          <h2 className="text-4xl font-black tracking-tighter italic">RECALL SESSION</h2>
          <p className="text-[#b3b3b3] text-sm mt-2 font-medium uppercase tracking-[0.2em]">
             {gameTitle ? `FSRS • ${gameTitle}` : 'Based on FSRS Algorithm'}
          </p>
       </header>

       <div 
         onClick={() => setFlipped(!flipped)}
         className="aspect-[4/3] perspective-1000 cursor-pointer group"
       >
          <div className={`relative w-full h-full transition-all duration-700 transform-style-3d ${flipped ? 'rotate-y-180' : ''}`}>
             {/* Front */}
             <div className="absolute inset-0 backface-hidden bg-[#181818] rounded-[2rem] p-12 flex flex-col items-center justify-center text-center border border-white/5 shadow-2xl group-hover:border-white/10">
                <span className="text-[10px] font-bold text-[#1ed760] uppercase tracking-[0.3em] mb-6">Concept Retrieval</span>
                <p className="text-3xl font-black leading-tight tracking-tight">{currentCard.front}</p>
                <div className="mt-12 flex items-center gap-2 text-white/20 font-bold uppercase text-[10px]">
                   <RotateCcw size={14} /> Click to Flip
                </div>
             </div>
             {/* Back */}
             <div className="absolute inset-0 backface-hidden bg-[#222] rounded-[2rem] p-12 flex flex-col items-center justify-center text-center border border-[#1ed760]/30 shadow-2xl rotate-y-180">
                <span className="text-[10px] font-bold text-[#1ed760] uppercase tracking-[0.3em] mb-6">Definition</span>
                <p className="text-2xl font-medium leading-relaxed text-white/90">{currentCard.back}</p>
             </div>
          </div>
       </div>

       {flipped && (
         <div className="mt-12 grid grid-cols-4 gap-4 animate-in slide-in-from-top-4 duration-500">
            <RatingPill label="Again" time="<1m" color="bg-red-500" onClick={() => handleRating(1)} />
            <RatingPill label="Hard" time="2d" color="bg-orange-500" onClick={() => handleRating(2)} />
            <RatingPill label="Good" time="4d" color="bg-green-500" onClick={() => handleRating(3)} />
            <RatingPill label="Easy" time="8d" color="bg-blue-500" onClick={() => handleRating(4)} />
         </div>
       )}
    </div>
  );
}

function RatingPill({ label, time, color, onClick }) {
  return (
    <button 
      onClick={(e) => { e.stopPropagation(); onClick(); }}
      className="bg-[#181818] p-4 rounded-2xl hover:bg-[#282828] transition-all border border-transparent hover:border-white/10 active:scale-95 text-center group"
    >
      <div className={`w-2 h-2 rounded-full mx-auto mb-2 ${color} shadow-[0_0_8px_rgba(0,0,0,0.5)]`} />
      <p className="font-bold text-xs group-hover:text-white transition-colors">{label}</p>
      <p className="text-[9px] text-[#b3b3b3] uppercase mt-1">{time}</p>
    </button>
  );
}

/**
 * --------------------------------------------------------------------------------
 * 5. Main Application Logic
 * --------------------------------------------------------------------------------
 */
export default function App() {
  const [view, setView] = useState('library'); // library, game_home, chapter_home, learning, review, bank
  const [activeGameId, setActiveGameId] = useState(null);
  const [activeChapterId, setActiveChapterId] = useState(null);
  const [selectedTerm, setSelectedTerm] = useState(null);
  const [showTermWiki, setShowTermWiki] = useState(false);

  useEffect(() => {
    if (selectedTerm) setShowTermWiki(true);
  }, [selectedTerm]);

  const activeGame = MOCK_GAMES.find(g => g.id === activeGameId);
  const activeChapter = activeGame?.chapters.find(c => c.id === activeChapterId);

  // 模拟从 Loader 注入的当前 Step 数据
  const mockStepData = {
    sequence_id: 'step1',
    term_catalog: {
      'scrape': { display: '网络爬取', gloss: '从网站自动提取数据的过程，通常涉及模拟请求与 HTML 解析。' },
      'dom': { display: 'DOM 树', gloss: '文档对象模型，是 HTML 和 XML 文档的编程接口。' }
    },
    screens: [
      { type: 'narration', lines: ['在这个快速变化的金融市场中，信息就是金钱。', '而获取信息最高效的方式，就是 <term id="scrape">网络爬取</term>。'] },
      { type: 'narration', lines: ['我们需要深入网页的 <term id="dom">DOM 树</term>，从中精准定位我们需要的数据点。'] },
      { type: 'exercise', exercise: { prompt: '以下哪个工具最适合进行基础的 HTML 解析？', options: ['Pandas', 'BeautifulSoup', 'Matplotlib'] }, lines: ['在我们开始之前，先看看你对工具有多了解。'] }
    ]
  };

  return (
    <div className="min-h-screen bg-black text-white selection:bg-[#1ed760] selection:text-black flex flex-col md:flex-row overflow-hidden font-sans">
      
      {/* 1. Spotify 风格侧边栏 */}
      <aside className="w-full md:w-64 bg-black p-4 flex flex-col gap-2 border-r border-[#181818] z-30">
        <div className="p-4 mb-6 flex items-center gap-2">
          <Layers className="text-[#1ed760]" size={32} />
          <span className="font-black text-2xl tracking-tighter">LEARNER</span>
        </div>

        <nav className="flex flex-col gap-1">
          <NavItem active={view === 'library'} icon={LayoutGrid} onClick={() => { setView('library'); setActiveGameId(null); setActiveChapterId(null); }}>全部游戏仓库</NavItem>
        </nav>

        {activeGame && (
          <div className="mt-6 flex flex-col gap-1">
            <p className="px-4 text-[10px] font-bold text-[#1ed760] uppercase tracking-widest mb-2 border-b border-white/5 pb-2">
              当前载入: {activeGame.title}
            </p>
            <NavItem active={view === 'game_home' || view === 'chapter_home'} icon={BookOpen} onClick={() => { setView('game_home'); setActiveChapterId(null); }}>路线与章节选择</NavItem>
            <NavItem active={view === 'review'} icon={Brain} onClick={() => setView('review')}>
               FSRS 记忆巩固
               {activeGame.itemsToReview > 0 && <span className="ml-auto w-5 h-5 bg-[#1ed760] rounded-full text-[10px] text-black flex items-center justify-center font-bold">{activeGame.itemsToReview}</span>}
            </NavItem>
            <NavItem active={view === 'bank'} icon={Target} onClick={() => setView('bank')}>综合题库图鉴</NavItem>
          </div>
        )}

        <div className="mt-auto overflow-y-auto pt-8">
          <p className="px-4 text-[10px] font-bold text-[#b3b3b3] uppercase tracking-widest mb-4">快捷跳转 (My Games)</p>
          {MOCK_GAMES.map(g => (
            <div 
              key={g.id} 
              onClick={() => { setActiveGameId(g.id); setActiveChapterId(null); setView('game_home'); }}
              className={`flex items-center gap-3 px-4 py-2.5 rounded-lg cursor-pointer transition-all ${activeGameId === g.id ? 'bg-[#282828] text-white shadow-lg' : 'text-[#b3b3b3] hover:text-white hover:bg-white/5'}`}
            >
              <div className="w-9 h-9 rounded-md flex items-center justify-center font-black text-xs shadow-inner" style={{ backgroundColor: g.color }}>{g.id}</div>
              <div className="overflow-hidden w-full">
                 <p className="text-sm font-bold truncate">{g.title}</p>
                 <div className="h-0.5 w-full bg-white/10 rounded-full mt-1">
                    <div className="h-full bg-[#1ed760]" style={{ width: g.mastery + '%' }} />
                 </div>
              </div>
            </div>
          ))}
        </div>
      </aside>

      {/* 2. 主内容区域 */}
      <main className="flex-1 bg-gradient-to-b from-[#1a1a1a] to-[#121212] overflow-y-auto relative pb-20">
        {/* 透明粘性头部 */}
        <header className="sticky top-0 h-20 bg-black/40 backdrop-blur-xl z-20 flex items-center justify-between px-10">
           <div className="flex gap-4">
              <button onClick={() => { if (view === 'chapter_home') setView('game_home'); else setView('library'); }} className="bg-black/60 p-2 rounded-full text-[#b3b3b3] hover:text-white transition-all"><ChevronLeft size={20}/></button>
              <button className="bg-black/60 p-2 rounded-full text-[#b3b3b3] hover:text-white transition-all"><ChevronRight size={20}/></button>
           </div>
           <div className="flex items-center gap-4">
              <PillButton variant="ghost" icon={Search}>搜索</PillButton>
              <div className="w-10 h-10 rounded-full bg-[#282828] border border-white/10 flex items-center justify-center font-bold text-xs">U</div>
           </div>
        </header>

        <div className="px-8 md:px-12 py-4 max-w-7xl mx-auto">
          
          {/* === LEVEL 1: LIBRARY VIEW (选游戏) === */}
          {view === 'library' && (
            <div className="animate-in fade-in duration-1000">
               <div className="flex items-end gap-3 mb-8">
                  <h2 className="text-4xl font-black italic tracking-tighter uppercase">Pick up your game</h2>
               </div>
               <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-6">
                 {MOCK_GAMES.map(g => <GameCard key={g.id} game={g} onPlay={() => { setActiveGameId(g.id); setActiveChapterId(null); setView('game_home'); }} />)}
               </div>
            </div>
          )}

          {/* === LEVEL 2: GAME OVERVIEW (选章节 L1~LN) === */}
          {view === 'game_home' && activeGame && (
            <div className="animate-in slide-in-from-bottom-6 duration-700 space-y-16">
               <div className="flex flex-col md:flex-row gap-10 items-end">
                  <div className="w-64 h-64 shadow-[0_48px_80px_-16px_rgba(0,0,0,0.8)] flex items-center justify-center text-9xl font-black text-black/10 rounded-xl" style={{ backgroundColor: activeGame.color }}>{activeGame.id}</div>
                  <div className="space-y-6">
                    <p className="text-xs font-black uppercase tracking-[0.4em] text-[#1ed760] flex items-center gap-2">
                       <Award size={16} /> Verified Course Game
                    </p>
                    <h1 className="text-6xl md:text-8xl font-black tracking-tighter leading-none">{activeGame.title}</h1>
                    <div className="flex items-center gap-8 text-sm text-[#b3b3b3] font-bold uppercase tracking-widest">
                       <span className="flex items-center gap-2 text-white"><Clock size={16}/> 包含 {activeGame.chapters.length} 个章节路线</span>
                       <span className="flex items-center gap-2"><Flame size={16} className="text-orange-500"/> 综合探索度 {activeGame.mastery}%</span>
                    </div>
                  </div>
               </div>

               <div className="space-y-4">
                  <h3 className="text-xl font-bold border-b border-white/10 pb-4">选择路线 (Chapters)</h3>
                  <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    {activeGame.chapters.map((chap) => (
                      <div 
                        key={chap.id} 
                        onClick={() => { setActiveChapterId(chap.id); setView('chapter_home'); }}
                        className="bg-[#181818]/60 border border-white/5 p-6 rounded-2xl hover:bg-[#282828] hover:border-[#1ed760]/30 transition-all cursor-pointer group"
                      >
                        <div className="flex justify-between items-start mb-4">
                           <span className="text-[#1ed760] font-black text-2xl">{chap.id}</span>
                           <Play className="text-[#b3b3b3] group-hover:text-white transition-colors" size={24} />
                        </div>
                        <h4 className="text-xl font-bold mb-2 group-hover:text-[#1ed760] transition-colors">{chap.title}</h4>
                        <p className="text-xs text-[#b3b3b3] mb-4">包含 {chap.steps.length} 个交互幕</p>
                        <div className="h-1.5 w-full bg-white/10 rounded-full overflow-hidden">
                           <div className="h-full bg-[#1ed760]" style={{ width: chap.progress + '%' }} />
                        </div>
                      </div>
                    ))}
                  </div>
               </div>
            </div>
          )}

          {/* === LEVEL 3: CHAPTER VIEW (选 Step / 独立课本) === */}
          {view === 'chapter_home' && activeChapter && (
            <div className="animate-in slide-in-from-right duration-500 space-y-12">
               <div className="space-y-6">
                  <p className="text-sm font-bold text-[#1ed760] uppercase tracking-widest flex items-center gap-2">
                    <ChevronLeft className="cursor-pointer hover:text-white" onClick={() => setView('game_home')} />
                    {activeGame.title}
                  </p>
                  <h2 className="text-5xl font-black tracking-tighter">{activeChapter.id}: {activeChapter.title}</h2>
                  
                  <div className="flex gap-4 pt-4">
                    <PillButton onClick={() => setView('learning')} icon={Play}>开始剧情 (Start Route)</PillButton>
                    <PillButton variant="outline" icon={BookOpen}>阅读本章独立课本 (Textbook)</PillButton>
                  </div>
               </div>

               <div className="space-y-1 bg-[#181818]/40 rounded-2xl p-6 border border-white/5">
                  <div className="grid grid-cols-[3rem_1fr_auto_8rem] px-4 py-3 border-b border-white/5 text-[#b3b3b3] font-bold text-[10px] uppercase tracking-widest mb-4">
                     <span>#</span>
                     <span>Scene / Step</span>
                     <span className="text-right">Type</span>
                     <span className="text-right">Status</span>
                  </div>
                  {activeChapter.steps.map((step, i) => (
                    <div 
                      key={i} 
                      onClick={() => setView('learning')}
                      className="grid grid-cols-[3rem_1fr_auto_8rem] items-center px-4 py-4 hover:bg-white/5 rounded-xl transition-all group cursor-pointer"
                    >
                      <span className="text-sm font-bold text-[#b3b3b3] group-hover:text-white">{i + 1}</span>
                      <div className="font-bold text-white group-hover:text-[#1ed760] transition-colors">{step.concept}</div>
                      <div className="text-right px-4">
                         {i === activeChapter.steps.length - 1 ? <Target size={16} className="text-[#333] ml-auto"/> : <MessageSquare size={16} className="text-[#333] ml-auto"/>}
                      </div>
                      <div className="flex items-center justify-end gap-3">
                         <CheckCircle2 size={18} className={activeChapter.progress === 100 || i === 0 ? "text-[#1ed760]" : "text-[#333]"} />
                      </div>
                    </div>
                  ))}
               </div>
            </div>
          )}

          {/* === SPECIFIC EXTRA: FSRS REVIEW === */}
          {view === 'review' && activeGame && (
             <FSRSReviewCenter 
               cards={INITIAL_FLASHCARDS} 
               onComplete={() => setView('game_home')} 
               gameTitle={activeGame.title}
             />
          )}

          {/* === SPECIFIC EXTRA: VAULT VIEW === */}
          {view === 'bank' && activeGame && (
             <div className="animate-in fade-in duration-700">
                <div className="flex items-center justify-between mb-12">
                   <div>
                     <h2 className="text-4xl font-black italic tracking-tighter uppercase">The Vault</h2>
                     <p className="text-[#b3b3b3] font-bold mt-2 uppercase tracking-widest text-xs">特供题库图鉴 • {activeGame.title}</p>
                   </div>
                   <div className="flex gap-2">
                      <PillButton variant="ghost" icon={Filter}>筛选</PillButton>
                      <PillButton variant="secondary" icon={Zap}>测验突击</PillButton>
                   </div>
                </div>
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                   <VaultItem title="基础术语检索 (Flashcards)" count={24} type="Memory" status="Due Soon" />
                   <VaultItem title="全章节选择题汇编 (Quizzes)" count={12} type="Test" status="Mastered" />
                   <VaultItem title="复杂推导与简答 (Longform)" count={6} type="Writing" status="Learning" />
                </div>
             </div>
          )}
        </div>
      </main>

      {/* 3. Galgame 模式 Overlay */}
      {view === 'learning' && activeChapter && (
        <GalgamePlayer 
          game={activeGame}
          chapter={activeChapter}
          stepData={mockStepData} 
          onExit={() => setView('chapter_home')} 
          setSelectedTerm={setSelectedTerm}
        />
      )}

      {/* 4. Wiki 风格术语侧边栏 */}
      {showTermWiki && selectedTerm && (
        <div className="fixed inset-0 z-[200] flex justify-end bg-black/60 backdrop-blur-sm animate-in fade-in duration-300">
          <div className="w-full max-w-sm bg-[#121212] h-full shadow-2xl flex flex-col border-l border-[#1ed760]/20 animate-in slide-in-from-right duration-500">
            <div className="p-8 flex-1 overflow-y-auto">
              <button 
                onClick={() => setShowTermWiki(false)} 
                className="mb-8 p-2 bg-white/5 rounded-full text-[#b3b3b3] hover:text-white transition-all"
              >
                <ChevronRight size={24}/>
              </button>
              
              <div className="space-y-8">
                 <div className="space-y-2">
                    <span className="text-[10px] font-black text-[#1ed760] uppercase tracking-[0.4em]">Glossary Entry</span>
                    <h4 className="text-4xl font-black italic tracking-tighter uppercase leading-none">{selectedTerm.display}</h4>
                 </div>
                 
                 <div className="p-6 bg-white/5 rounded-2xl border border-white/5">
                    <p className="text-lg leading-relaxed text-white/80 font-medium">
                      {selectedTerm.gloss}
                    </p>
                 </div>

                 <div className="space-y-4">
                    <h5 className="text-xs font-bold uppercase tracking-widest text-[#b3b3b3]">Related Concepts</h5>
                    <div className="flex flex-wrap gap-2">
                       {['BeautifulSoup', 'Scrapy', 'HTTP', 'JSON'].map(tag => (
                         <span key={tag} className="px-3 py-1 bg-white/5 rounded-full text-[10px] font-bold border border-white/5">#{tag}</span>
                       ))}
                    </div>
                 </div>
              </div>
            </div>
            <div className="p-8 border-t border-white/5">
               <PillButton variant="primary" className="w-full" onClick={() => setShowTermWiki(false)}>返回剧情</PillButton>
        </div>
      </div>
    </div>
  )}

  <style>{`
    .perspective-1000 { perspective: 1000px; }
    .backface-hidden { backface-visibility: hidden; }
    .transform-style-3d { transform-style: preserve-3d; }
    .rotate-y-180 { transform: rotateY(180deg); }
  `}</style>
</div>
);
}

/**
 * --------------------------------------------------------------------------------
 * 6. UI Detail Components
 * --------------------------------------------------------------------------------
 */

function NavItem({ children, icon: Icon, active, onClick }) {
  return (
    <button 
      onClick={onClick}
      className={`flex items-center gap-4 px-4 py-3 font-bold transition-all rounded-xl ${active ? 'bg-white/10 text-[#1ed760]' : 'text-[#b3b3b3] hover:text-white hover:bg-white/5'}`}
    >
      <Icon size={24} strokeWidth={active ? 3 : 2} />
      <span className="text-[13px] tracking-tight">{children}</span>
    </button>
  );
}

function GameCard({ game, onPlay }) {
  return (
    <div 
      onClick={onPlay}
      className="bg-[#181818] p-5 rounded-2xl hover:bg-[#282828] transition-all group cursor-pointer shadow-xl relative border border-white/5"
    >
      <div className="aspect-square rounded-xl mb-4 flex items-center justify-center relative shadow-2xl overflow-hidden" style={{ backgroundColor: game.color }}>
        <span className="text-7xl font-black text-black/10 select-none tracking-tighter">{game.id}</span>
        <div className="absolute inset-0 bg-gradient-to-br from-white/20 to-transparent" />
        <div className="absolute bottom-4 right-4 w-14 h-14 bg-[#1ed760] rounded-full flex items-center justify-center text-black opacity-0 group-hover:opacity-100 group-hover:translate-y-0 translate-y-4 transition-all shadow-[0_8px_24px_rgba(30,215,96,0.4)]">
           <Play fill="black" size={24} />
        </div>
      </div>
      <h3 className="font-black text-white truncate mb-1 tracking-tight">{game.title}</h3>
      <div className="flex items-center justify-between mt-2">
         <span className="text-[10px] text-[#b3b3b3] font-bold uppercase tracking-widest">{game.category}</span>
         <span className="text-[10px] text-[#1ed760] font-bold">{game.mastery}%</span>
      </div>
    </div>
  );
}

function VaultItem({ title, count, type, status }) {
  return (
    <div className="bg-[#181818] p-6 rounded-2xl border border-white/5 hover:border-[#1ed760]/30 transition-all group cursor-pointer">
       <div className="flex items-start justify-between mb-6">
          <div className="p-3 bg-[#282828] rounded-xl text-[#b3b3b3] group-hover:text-[#1ed760] transition-colors">
             <Target size={24} />
          </div>
          <span className={`text-[9px] font-black uppercase tracking-[0.2em] px-2 py-1 rounded ${status === 'Due Soon' ? 'bg-[#f3727f]/10 text-[#f3727f]' : 'bg-white/10 text-[#b3b3b3]'}`}>
             {status}
          </span>
       </div>
       <h4 className="text-lg font-black tracking-tight mb-1 group-hover:text-[#1ed760] transition-colors">{title}</h4>
       <p className="text-xs text-[#b3b3b3] font-medium">{count} 张卡片 • {type}</p>
    </div>
  );
}

const INITIAL_FLASHCARDS = [
  { id: 'c1', front: '什么是网络爬取（Web Scraping）？', back: '从网站自动提取数据的过程，通常用于替代繁琐的手动数据收集。', stability: 5.2, difficulty: 3.1, state: 'review' },
  { id: 'c2', front: 'DOM 的全称是什么？', back: 'Document Object Model（文档对象模型）。', stability: 12.0, difficulty: 2.1, state: 'mastered' },
  { id: 'c3', front: '为什么算法交易需要爬虫？', back: '为了获取实时新闻、财报或社交媒体情绪等另类数据，作为策略的输入。', stability: 1.1, difficulty: 5.5, state: 'learning' },
];