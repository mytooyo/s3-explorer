export const createResizable = (item: HTMLElement, resizer: HTMLDivElement) => {
  var x = 0;
  var w = 0;

  const mouseDownHandler = (e: MouseEvent) => {
    // 現在のマウスポジション取得
    x = e.clientX;

    // 現在の横幅を取得
    const styles = window.getComputedStyle(item);
    w = parseInt(styles.width, 10);

    // リスナーを登録
    document.addEventListener('mousemove', mouseMoveHandler);
    document.addEventListener('mouseup', mouseUpHandler);
    resizer.classList.add('resizing');
  };

  const mouseMoveHandler = (e: MouseEvent) => {
    const dx = e.clientX - x;
    item.style.width = `${w + dx}px`;
  };

  const mouseUpHandler = (e: MouseEvent) => {
    document.removeEventListener('mousemove', mouseMoveHandler);
    document.removeEventListener('mouseup', mouseUpHandler);
    resizer.classList.remove('resizing');
  };

  resizer.addEventListener('mousedown', mouseDownHandler);
};
