import matplotlib.pyplot as plt # type: ignore
import numpy as np # type: ignore

powers_of_ten = [r'$10^1$', r'$10^2$', r'$10^3$', r'$10^4$', r'$10^5$']

# Performance Gap Results
arm_AcqRel_vs_Relaxed = [0.68, 1.51, 5.03, 6.24, 7.18]
arm_86_SeqCst_vs_Relaxed = [1.62, 2.59, 7.94, 9.48, 12.54]
arm_SeqCst_vs_AcqRel = [0.93, 1.06, 2.77, 2.86, 5.00]


bar_width = 0.22

intra_group_spacing = 0.03

inter_group_spacing = 0.4

color_blue = (188/255, 215/255, 213/255)
color_green = (197/255, 226/255, 135/255)
color_pink = (237/255, 172/255, 175/255)


group_width = 3 * bar_width + 2 * intra_group_spacing
positions = np.arange(len(powers_of_ten))  # 基本位置
positions_1 = positions - bar_width - intra_group_spacing
positions_2 = positions
positions_3 = positions + bar_width + intra_group_spacing

fig, ax = plt.subplots(figsize=(10, 7))

ax.bar(positions_1, arm_86_SeqCst_vs_Relaxed, width=bar_width, color=color_blue, edgecolor='black', label='SeqCst vs. Relaxed')
ax.bar(positions_2, arm_AcqRel_vs_Relaxed, width=bar_width, color=color_green, edgecolor='black', label='AcqRel vs. Relaxed')
ax.bar(positions_3, arm_SeqCst_vs_AcqRel, width=bar_width, color=color_pink, edgecolor='black', label='SeqCst vs. AcqRel')
ax.tick_params(axis='x', labelsize=14)
ax.tick_params(axis='y', labelsize=13)

ax.set_title('Performance Evaluation on ARM', fontsize=16)
ax.legend(fontsize=15)


ax.set_xlabel('Number of threads', fontsize=14)
ax.set_ylabel('Time Performance (%)', fontsize=13, labelpad=0)


ax.set_xticks(positions)
ax.set_xticklabels(powers_of_ten)


for i in range(len(positions)):
    ax.text(positions_1[i], arm_86_SeqCst_vs_Relaxed[i] + 0.1, f'{arm_86_SeqCst_vs_Relaxed[i]:.2f}', ha='center', va='bottom', fontsize=13)
    ax.text(positions_2[i], arm_AcqRel_vs_Relaxed[i] + 0.1, f'{arm_AcqRel_vs_Relaxed[i]:.2f}', ha='center', va='bottom', fontsize=13)
    ax.text(positions_3[i], arm_SeqCst_vs_AcqRel[i] + 0.1, f'{arm_SeqCst_vs_AcqRel[i]:.2f}', ha='center', va='bottom', fontsize=13)


plt.tight_layout()
plt.show()
fig.savefig("performance.png" , bbox_inches='tight')